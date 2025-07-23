/// The possible errors `libfdt` can output.
/// It is a 1-to-1 translation of error`libfdt` can issue.
#[derive(Debug, Clone)]
pub enum Error {
    /// The requested node or property does not exist
    NotFound,
    /// Attempted to create a node or property which already exists
    Exists,
    /// Operation needed to expand the device
    /// tree, but its buffer did not have sufficient space to
    /// contain the expanded tree.
    NoSpace,
    /// Function was passed a structure block
    /// offset which is out-of-bounds, or which points to an
    /// unsuitable part of the structure for the operation.
    BadOffset,
    /// Function was passed a badly formatted path
    /// (e.g. missing a leading / for a function which requires an
    /// absolute path)
    BadPath,
    /// Function was passed an invalid phandle.
    /// This can be caused either by an invalid phandle property
    /// length, or the phandle value was either 0 or -1, which are
    /// not permitted.
    BadPhandle,
    /// Function was passed an incomplete device
    /// tree created by the sequential-write functions, which is
    /// not sufficiently complete for the requested operation.
    BadState,
    Truncated,
    BadMagic,
    BadVersion,
    BadStructure,
    BadLayout,
    Internal,
    BadNCells,
    BadValue,
    BadOverlay,
    NoPhandle,
    BadFlags,
    Alignment,
    Unknown(i32),
}

impl Error {
    pub fn parse(ret: i32) -> Result<i32, Error> {
        if ret >= 0 {
            return Ok(ret);
        }

        let err = u32::try_from(-ret).unwrap();

        match err {
            libfdt_sys::FDT_ERR_NOTFOUND => Err(Error::NotFound),
            libfdt_sys::FDT_ERR_EXISTS => Err(Error::Exists),
            libfdt_sys::FDT_ERR_NOSPACE => Err(Error::NoSpace),
            libfdt_sys::FDT_ERR_BADOFFSET => Err(Error::BadOffset),
            libfdt_sys::FDT_ERR_BADPATH => Err(Error::BadPath),
            libfdt_sys::FDT_ERR_BADPHANDLE => Err(Error::BadPhandle),
            libfdt_sys::FDT_ERR_BADSTATE => Err(Error::BadState),
            libfdt_sys::FDT_ERR_TRUNCATED => Err(Error::Truncated),
            libfdt_sys::FDT_ERR_BADMAGIC => Err(Error::BadMagic),
            libfdt_sys::FDT_ERR_BADVERSION => Err(Error::BadVersion),
            libfdt_sys::FDT_ERR_BADSTRUCTURE => Err(Error::BadStructure),
            libfdt_sys::FDT_ERR_BADLAYOUT => Err(Error::BadLayout),
            libfdt_sys::FDT_ERR_INTERNAL => Err(Error::Internal),
            libfdt_sys::FDT_ERR_BADNCELLS => Err(Error::BadNCells),
            libfdt_sys::FDT_ERR_BADVALUE => Err(Error::BadValue),
            libfdt_sys::FDT_ERR_BADOVERLAY => Err(Error::BadOverlay),
            libfdt_sys::FDT_ERR_NOPHANDLES => Err(Error::NoPhandle),
            libfdt_sys::FDT_ERR_BADFLAGS => Err(Error::BadFlags),
            libfdt_sys::FDT_ERR_ALIGNMENT => Err(Error::Alignment),
            _ => Err(Error::Unknown(ret)),
        }
    }
}
