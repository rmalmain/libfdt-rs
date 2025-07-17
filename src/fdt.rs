use crate::{
    Error, FdtNode, FdtNodeRef, FdtProperty, PHANDLE_LINKS_SIMPLE, PHANDLE_LINKS_SUFFIX,
    PhandleLink,
};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::fmt::{Debug, Formatter};
use core::mem::MaybeUninit;
use core::ops::DerefMut;
use core::pin::Pin;

#[cfg(not(feature = "std"))]
use alloc::{
    boxed::Box,
    ffi::CString,
    string::{String, ToString},
};

#[cfg(feature = "std")]
use std::{
    boxed::Box,
    ffi::CString,
    str::FromStr,
    string::{String, ToString},
};

#[cfg(not(feature = "std"))]
use core::str::FromStr;

#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeSet as HashSet, vec::Vec};

use core::fmt;
#[cfg(feature = "std")]
use std::{collections::HashSet, vec::Vec};

pub struct Fdt {
    _inner: Pin<Box<[u8]>>,
    // inner is pinned, so we can store a raw pointer to the fdt safely.
    pub(crate) fdt: *mut c_void,
    pub(crate) links_simple: HashSet<PhandleLink>,
    pub(crate) links_suffix: Vec<PhandleLink>,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Offset(pub(crate) c_int);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Phandle(u32);

impl Debug for Fdt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<fdt>")
    }
}

impl TryFrom<u32> for Phandle {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 || value > libfdt_sys::FDT_MAX_PHANDLE {
            Err(Error::BadPhandle)
        } else {
            Ok(Phandle(value))
        }
    }
}

impl Fdt {
    pub fn new(fdt: &[u8]) -> Result<Fdt, Error> {
        let mut inner: Pin<Box<[u8]>> = Pin::new(Box::from(fdt));
        let fdt: *mut c_void = inner.deref_mut().as_mut_ptr() as *mut c_void;

        unsafe {
            Error::parse(libfdt_sys::fdt_check_header(fdt))?;
        }

        let links_simple: HashSet<PhandleLink> = PHANDLE_LINKS_SIMPLE
            .iter()
            .flat_map(|links| links.iter())
            .cloned()
            .collect();

        let links_suffix: Vec<PhandleLink> = PHANDLE_LINKS_SUFFIX
            .iter()
            .flat_map(|links| links.iter())
            .cloned()
            .collect();

        Ok(Self {
            _inner: inner,
            fdt,
            links_simple,
            links_suffix,
        })
    }

    pub fn path_offset(&self, path: &str) -> Result<Offset, Error> {
        let path_cstr = CString::from_str(path).unwrap();

        unsafe {
            Ok(Offset(Error::parse(libfdt_sys::fdt_path_offset(
                self.fdt,
                path_cstr.as_ptr(),
            ))?))
        }
    }

    pub fn first_property_offset(&self, nodeoffset: Offset) -> Result<Offset, Error> {
        unsafe {
            Ok(Offset(Error::parse(
                libfdt_sys::fdt_first_property_offset(self.fdt, nodeoffset.0),
            )?))
        }
    }

    pub fn first_property<'fdt>(
        &'fdt self,
        node: &FdtNode<'fdt>,
    ) -> Result<Option<FdtProperty<'fdt>>, Error> {
        match self.first_property_offset(node.offset) {
            Ok(offset) => Ok(Some(self.get_property_by_offset(offset)?)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn next_property_offset(&self, offset: Offset) -> Result<Offset, Error> {
        unsafe {
            Ok(Offset(Error::parse(libfdt_sys::fdt_next_property_offset(
                self.fdt, offset.0,
            ))?))
        }
    }

    pub fn next_property<'fdt>(
        &'fdt self,
        property: &FdtProperty<'fdt>,
    ) -> Result<Option<FdtProperty<'fdt>>, Error> {
        match self.next_property_offset(property.offset.unwrap()) {
            Ok(offset) => Ok(Some(self.get_property_by_offset(offset)?)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn first_subnode_offset(&self, offset: Offset) -> Result<Offset, Error> {
        unsafe {
            Ok(Offset(Error::parse(libfdt_sys::fdt_first_subnode(
                self.fdt, offset.0,
            ))?))
        }
    }

    pub fn first_subnode<'fdt>(
        &'fdt self,
        parent_node: &FdtNode<'fdt>,
    ) -> Result<Option<FdtNode<'fdt>>, Error> {
        match self.first_subnode_offset(parent_node.offset) {
            Ok(offset) => Ok(Some(self.get_node_by_offset(offset)?)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn next_subnode_offset(&self, offset: Offset) -> Result<Offset, Error> {
        unsafe {
            Ok(Offset(Error::parse(libfdt_sys::fdt_next_subnode(
                self.fdt, offset.0,
            ))?))
        }
    }

    pub fn next_subnode<'fdt>(
        &'fdt self,
        previous_node: &FdtNode<'fdt>,
    ) -> Result<Option<FdtNode<'fdt>>, Error> {
        match self.next_subnode_offset(previous_node.offset) {
            Ok(offset) => Ok(Some(self.get_node_by_offset(offset)?)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn get_node_by_offset<'fdt>(
        &'fdt self,
        nodeoffset: Offset,
    ) -> Result<FdtNode<'fdt>, Error> {
        let mut len: c_int = 0;

        let name = unsafe { libfdt_sys::fdt_get_name(self.fdt, nodeoffset.0, &raw mut len) };

        if name.is_null() {
            return Err(Error::parse(len).unwrap_err());
        }

        let name = unsafe { CStr::from_ptr(name) };

        Ok(FdtNode {
            offset: nodeoffset,
            fdt: self,
            name,
        })
    }

    pub fn get_property_by_offset<'fdt>(
        &'fdt self,
        offset: Offset,
    ) -> Result<FdtProperty<'fdt>, Error> {
        let mut len: c_int = 0;
        let mut name: MaybeUninit<*const c_char> = MaybeUninit::uninit();

        let prop_ptr = unsafe {
            libfdt_sys::fdt_getprop_by_offset(self.fdt, offset.0, name.as_mut_ptr(), &raw mut len)
        };

        if prop_ptr.is_null() {
            return Err(Error::parse(len).unwrap_err());
        }

        let name = unsafe { name.assume_init() };
        let name = unsafe { CString::from(CStr::from_ptr(name)) };

        Ok(FdtProperty {
            fdt: self,
            data: prop_ptr,
            len,
            name,
            offset: Some(offset),
        })
    }

    pub fn get_property<'fdt>(
        &'fdt self,
        node: &FdtNode<'fdt>,
        property_name: &str,
    ) -> Result<FdtProperty<'fdt>, Error> {
        let mut len: c_int = 0;
        let name = CString::from_str(property_name).unwrap();

        let prop_ptr = unsafe {
            libfdt_sys::fdt_getprop(self.fdt, node.offset.0, name.as_ptr(), &raw mut len)
        };

        if prop_ptr.is_null() {
            return Err(Error::parse(len).unwrap_err());
        }

        Ok(FdtProperty {
            data: prop_ptr,
            fdt: self,
            len,
            name,
            offset: None,
        })
    }

    pub fn get_phandle<'fdt>(&'fdt self, node: &FdtNode<'fdt>) -> Result<Phandle, Error> {
        unsafe { Phandle::try_from(libfdt_sys::fdt_get_phandle(self.fdt, node.offset.0)) }
    }

    pub fn is_compatible<'fdt>(
        &'fdt self,
        node: &FdtNode<'fdt>,
        compatible: &str,
    ) -> Result<bool, Error> {
        let compatible_str = CString::from_str(compatible).unwrap();

        let res = unsafe {
            Error::parse(libfdt_sys::fdt_node_check_compatible(
                self.fdt,
                node.offset.0,
                compatible_str.as_ptr(),
            ))?
        };

        Ok(res == 0)
    }

    pub fn get_node<'fdt>(&'fdt self, path: &str) -> Result<FdtNode<'fdt>, Error> {
        let path_str = CString::from_str(path).unwrap();

        let offset: Offset = unsafe {
            Offset(Error::parse(libfdt_sys::fdt_path_offset(
                self.fdt,
                path_str.as_ptr(),
            ))?)
        };

        self.get_node_by_offset(offset)
    }

    pub fn get_node_by_phandle<'fdt>(
        &'fdt self,
        phandle: &Phandle,
    ) -> Result<FdtNode<'fdt>, Error> {
        let nodeoffset =
            unsafe { Error::parse(libfdt_sys::fdt_node_offset_by_phandle(self.fdt, phandle.0))? };

        self.get_node_by_offset(Offset(nodeoffset))
    }

    pub fn as_path<'fdt>(&'fdt self, node_ref: &'fdt FdtNodeRef) -> Result<&'fdt str, Error> {
        match node_ref {
            FdtNodeRef::Path(path) => Ok(path.as_str()),
            FdtNodeRef::Symbol(symbol) => {
                let snode = self.get_node("/__symbols__")?;
                let sprop = snode.get_property(symbol)?;
                unsafe { Ok(sprop.data_as_str()) }
            }
        }
    }

    pub fn symbols(&self) -> Result<HashSet<String>, Error> {
        let mut symbols = HashSet::new();
        let snode = self.get_node("/__symbols__")?;
        for prop in snode.properties_iter()? {
            let s = unsafe { prop.data_as_str() };
            symbols.insert(s.to_string());
        }

        Ok(symbols)
    }
}
