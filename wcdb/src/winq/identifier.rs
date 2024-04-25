use std::{ffi::CStr, mem::size_of};

use libwcdb_sys::CPPObject;

pub struct Identifier<T>(T)
where
    T: Into<*mut CPPObject> + Clone + Copy;

impl<T> Identifier<T>
where
    T: Into<*mut CPPObject> + Clone + Copy,
{
    // This is a compile-time check to ensure that the size of the type `T` is the same as the size of a pointer.
    #[allow(dead_code)]
    const SIZE_OK: () = assert!(size_of::<T>() == size_of::<*mut CPPObject>());

    pub fn new(value: T) -> Self {
        let _ = Self::SIZE_OK;
        Identifier(value)
    }

    pub fn as_ptr(&self) -> *mut CPPObject {
        self.0.into()
    }

    pub fn description(&self) -> String {
        let desc = unsafe { libwcdb_sys::WCDBWinqGetDescription(self.as_ptr()) };
        let desc = unsafe { CStr::from_ptr(desc) };
        desc.to_string_lossy().into_owned()
    }
}

impl<T> Drop for Identifier<T>
where
    T: Into<*mut CPPObject> + Clone + Copy,
{
    fn drop(&mut self) {
        unsafe {
            libwcdb_sys::WCDBReleaseCPPObject(self.as_ptr());
        }
    }
}

impl<T> From<T> for Identifier<T>
where
    T: Into<*mut CPPObject> + Clone + Copy,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}