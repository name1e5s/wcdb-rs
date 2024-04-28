use std::{ffi::CStr, mem::size_of};

use libwcdb_sys::CPPObject;

pub trait WithRawIdentifier<T> {
    fn with_raw<F, R>(&self, f: F) -> R
    where
        F: FnOnce(T) -> R;
}

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

impl<T> WithRawIdentifier<T> for Identifier<T>
where
    T: Into<*mut CPPObject> + Clone + Copy,
{
    fn with_raw<F, R>(&self, f: F) -> R
    where
        F: FnOnce(T) -> R,
    {
        f(self.0)
    }
}

impl<T, R> WithRawIdentifier<R> for &T
where
    T: WithRawIdentifier<R>,
{
    fn with_raw<F, R1>(&self, f: F) -> R1
    where
        F: FnOnce(R) -> R1,
    {
        T::with_raw(self, f)
    }
}

impl<T1, T2, R1, R2> WithRawIdentifier<(R1, R2)> for (T1, T2)
where
    T1: WithRawIdentifier<R1>,
    T2: WithRawIdentifier<R2>,
{
    fn with_raw<F, R>(&self, f: F) -> R
    where
        F: FnOnce((R1, R2)) -> R,
    {
        self.0.with_raw(|r1| self.1.with_raw(|r2| f((r1, r2))))
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
