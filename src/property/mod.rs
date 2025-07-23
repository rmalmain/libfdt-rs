//! # Properties
//!
//! Properties are the fields embedded in the nodes of the DT.

use crate::{Error, Fdt, FdtNode, Offset, Phandle};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;

#[cfg(not(feature = "std"))]
use alloc::{ffi::CString, vec::Vec};
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
#[cfg(feature = "std")]
use std::ffi::CString;

mod linux;
use linux::{LINUX_PHANDLE_PROPERTIES_SIMPLE_LIST, LINUX_PHANDLE_PROPERTIES_SUFFIX_LIST};

pub const PHANDLE_LINKS_SIMPLE: &[&[PhandleLink]] = &[LINUX_PHANDLE_PROPERTIES_SIMPLE_LIST];

pub const PHANDLE_LINKS_SUFFIX: &[&[PhandleLink]] = &[LINUX_PHANDLE_PROPERTIES_SUFFIX_LIST];

/// A property parser.
pub trait PropertyParser {
    /// The output type of the parser
    type Output;

    /// Parse a property's data
    ///
    /// # Safety
    ///
    /// Parsing a raw pointer is inherently unsafe, since it will at least
    /// require casting the pointer to some data type.
    /// The pointer should point to the right underlying type, and have
    /// sufficient size to contain a [`Self::Output`].
    unsafe fn parse(ptr: *const c_void) -> Self::Output;
}

/// A node property.
///
/// The underlying data depends on the property.
/// Various rules can apply to interpret the data correctly, depending on the property's name
/// or the node containing the property.
/// Some of these rules are described in the Device Tree specification, others depend on the
/// vendor or the underlying platform.
#[derive(Debug, Clone)]
pub struct FdtProperty<'fdt> {
    pub(crate) fdt: &'fdt Fdt,
    pub(crate) name: CString,
    pub(crate) data: *const c_void,
    pub(crate) len: c_int,
    pub(crate) offset: Option<Offset>,
}

/// A link between two nodes.
///
/// These links originate from phandle references.
/// The way these phandles are parsed is not fully specified by the `devicetree` specification.
/// Vendors, kernels, bootloaders can have different conventions when it comes to phandle parsing.
///
/// For now, only Linux kernel links are supported.
#[derive(Debug, Clone)]
pub struct PhandleLink {
    pub name: &'static str,
    pub size: &'static str,
}

/// A property reader, for cells.
pub struct PropertyCellParser;
impl PropertyParser for PropertyCellParser {
    type Output = u32;

    unsafe fn parse(ptr: *const c_void) -> u32 {
        let val = unsafe { *(ptr as *const u32) };
        u32::from_be(val)
    }
}

/// A property data reader.
///
/// It reads data from the beginning to the end.
/// Each time a call to [`PropertyReader::read`] is issued, the inner cursor will
/// advance by the size of the type being read.
pub struct PropertyReader<'fdt> {
    data: *const c_void,
    len: usize,
    pos: usize,
    phantom: PhantomData<&'fdt ()>,
}

impl PartialOrd for PhandleLink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PhandleLink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}

impl Borrow<str> for PhandleLink {
    fn borrow(&self) -> &'static str {
        self.name
    }
}

impl PartialEq for PhandleLink {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PhandleLink {}

impl Hash for PhandleLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl<'fdt> From<&FdtProperty<'fdt>> for PropertyReader<'fdt> {
    fn from(prop: &FdtProperty<'fdt>) -> Self {
        Self {
            pos: 0,
            data: prop.data,
            len: prop.len as usize,
            phantom: PhantomData,
        }
    }
}

impl<'fdt> PropertyReader<'fdt> {
    /// Reads a property as a P.
    /// If the remaining property size is smaller than the size of P,
    /// [`None`] is returned.
    ///
    /// # Safety
    ///
    /// P should indeed be the type contained by the property at the given
    /// offset.
    pub unsafe fn read<P>(&mut self) -> Option<P::Output>
    where
        P: PropertyParser,
    {
        if self.len - self.pos < size_of::<P::Output>() {
            return None;
        }

        let val_ptr = unsafe { self.data.add(self.pos) };

        self.pos += size_of::<P::Output>();

        Some(unsafe { P::parse(val_ptr) })
    }
}

impl<'fdt> FdtProperty<'fdt> {
    /// # Safety
    ///
    /// Cast the property's data as a string.
    pub unsafe fn data_as_str(&self) -> &'fdt str {
        unsafe {
            let cstr = CStr::from_ptr(self.data as *const c_char);
            cstr.to_str().unwrap()
        }
    }

    /// Get the name of the property.
    pub fn name(&self) -> &str {
        let cstr = self.name.as_c_str();
        cstr.to_str().unwrap()
    }

    /// Given a link name (as registered by [`Fdt`]), give the [`PhandleLink`] if there is one.
    /// If no link exists, return [`None`].
    fn get_link(&self, name: &str) -> Option<&PhandleLink> {
        if let Some(prop) = self.fdt.links_simple.get(name) {
            return Some(prop);
        }

        self.fdt
            .links_suffix
            .iter()
            .find(|suffix| name.ends_with(suffix.name))
    }

    /// Get a list of nodes linked to the property, if it is supposed to contain phandles.
    /// The [`Fdt`] in which the property lives contains the list of possible links.
    pub fn links(&self) -> Result<Option<Vec<FdtNode<'fdt>>>, Error> {
        let name = self.name();

        if let Some(phandle_prop) = self.get_link(name) {
            let mut res: Vec<FdtNode<'fdt>> = Vec::new();
            let mut rdr: PropertyReader = self.into();

            while let Some(phandle) = unsafe { rdr.read::<PropertyCellParser>() } {
                let phandle = match Phandle::try_from(phandle) {
                    Ok(phandle) => phandle,
                    Err(Error::BadPhandle) => {
                        log::warn!("Warning: invalid phandle {phandle}");
                        continue;
                    }
                    Err(e) => return Err(e),
                };

                let target_node = match self.fdt.get_node_by_phandle(&phandle) {
                    Ok(target_node) => target_node,
                    Err(Error::NoPhandle) => {
                        log::warn!("Warning: no phandle {phandle:?}");
                        continue;
                    }
                    Err(e) => return Err(e),
                };

                let size = if phandle_prop.size.is_empty() {
                    0
                } else {
                    let size_prop = match self.fdt.get_property(&target_node, phandle_prop.size) {
                        Ok(size_prop) => Some(size_prop),
                        Err(Error::NotFound) => {
                            log::warn!(
                                "Warning: no size property \"{}\"found for {}. Defaulting to 0...",
                                phandle_prop.size,
                                target_node.path()?
                            );
                            None
                        }
                        Err(e) => return Err(e),
                    };

                    if let Some(size_prop) = size_prop {
                        let mut size_prop_rdr: PropertyReader = (&size_prop).into();
                        unsafe { size_prop_rdr.read::<PropertyCellParser>() }.unwrap()
                    } else {
                        0
                    }
                };

                for _ in 0..size {
                    unsafe { rdr.read::<PropertyCellParser>() };
                }

                res.push(target_node.clone());
            }

            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}
