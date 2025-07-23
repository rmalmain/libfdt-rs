use crate::{Error, Fdt, FdtNodeIter, FdtProperty, FdtPropertyIter, Offset};

use core::borrow::Borrow;
use core::ffi::{CStr, c_char, c_int};
use core::hash::{Hash, Hasher};

#[cfg(feature = "std")]
use std::string::{String, ToString};

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

/// Node representation in an [`Fdt`].
#[derive(Debug, Clone)]
pub struct FdtNode<'fdt> {
    pub(crate) fdt: &'fdt Fdt,
    pub(crate) offset: Offset,
    pub(crate) name: &'fdt CStr,
}

/// A node reference in an [`Fdt`].
/// There are two possible references:
///     - [`FdtNodeRef::Path`]: a full path to a node.
///     - [`FdtNodeRef::Symbol`]: a symbol pointing to a node.
#[derive(Debug)]
pub enum FdtNodeRef {
    Path(String),
    Symbol(String),
}

impl<'fdt> PartialEq for FdtNode<'fdt> {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl<'fdt> Eq for FdtNode<'fdt> {}

impl<'fdt> Borrow<Offset> for FdtNode<'fdt> {
    fn borrow(&self) -> &Offset {
        &self.offset
    }
}

impl<'fdt> Hash for FdtNode<'fdt> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state)
    }
}

impl<'fdt> FdtNode<'fdt> {
    /// Get the [`Fdt`] in which the node lives.
    pub fn fdt(&self) -> &'fdt Fdt {
        self.fdt
    }

    /// Get the offset in the [`Fdt`] of the node.
    pub fn offset(&self) -> Offset {
        self.offset
    }

    /// Get an iterator over the subnodes of the node.
    pub fn subnodes_iter(&self) -> Result<FdtNodeIter<'fdt>, Error> {
        FdtNodeIter::new(self)
    }

    /// Get an iterator over the properties of the node.
    pub fn properties_iter(&self) -> Result<FdtPropertyIter<'fdt>, Error> {
        FdtPropertyIter::new(self)
    }

    /// Get the name of the node.
    pub fn name(&self) -> &str {
        self.name.to_str().unwrap()
    }

    /// Get the path in the [`Fdt`] of the node.
    pub fn path(&self) -> Result<String, Error> {
        let mut str_buf: [c_char; 2048] = [0; 2048];

        unsafe {
            Error::parse(libfdt_sys::fdt_get_path(
                self.fdt.fdt,
                self.offset.0,
                str_buf.as_mut_ptr(),
                str_buf.len() as c_int,
            ))?;
        }

        let c_str = unsafe { CStr::from_ptr(str_buf.as_ptr()) };
        let s = c_str.to_str().unwrap();

        Ok(s.to_string())
    }

    /// Get a property in the node given its name.
    pub fn get_property(&self, property_name: &str) -> Result<FdtProperty<'fdt>, Error> {
        self.fdt.get_property(self, property_name)
    }
}
