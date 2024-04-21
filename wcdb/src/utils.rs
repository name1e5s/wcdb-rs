macro_rules! cpp_bridged {
    ($name:ident) => {
        cpp_bridged!(pub struct $name(libwcdb_sys::$name));
    };

    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($raw:ty)
    ) => {
        $(#[$attr])*
        $vis struct $name($raw);

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    libwcdb_sys::WCDBReleaseCPPObject(self.0.into());
                }
            }
        }
    };
}

use crate::error::Result;
use std::{
    ffi::{CStr, CString},
    path::Path,
};

pub(crate) use cpp_bridged;

pub fn path_to_cstring(p: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(p.as_os_str().as_bytes())?)
}

pub fn c_ptr_to_string_opt(ptr: *const std::os::raw::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}
