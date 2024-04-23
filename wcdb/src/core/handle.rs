pub struct Handle {
    raw: libwcdb_sys::CPPHandle,
    owned: bool,
}

impl Handle {
    pub fn owned(raw: libwcdb_sys::CPPHandle) -> Self {
        Self { raw, owned: true }
    }

    pub fn reference(raw: libwcdb_sys::CPPHandle) -> Self {
        Self { raw, owned: false }
    }

    pub fn raw(&self) -> libwcdb_sys::CPPHandle {
        self.raw
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                libwcdb_sys::WCDBReleaseCPPObject(self.raw.into());
            }
        }
    }
}
