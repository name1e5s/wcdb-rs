use either::Either;

use crate::{
    error::{self, Result, WCDBError},
    utils::{cpp_bridged, path_to_cstring},
    Tag,
};
use std::{ffi::CString, path::Path};
use std::{ptr, sync::Arc};

use super::handle::Handle;

cpp_bridged!(struct InnerDatabase(libwcdb_sys::CPPDatabase));

unsafe impl Send for InnerDatabase {}
unsafe impl Sync for InnerDatabase {}

#[derive(Clone)]
pub struct Database(Arc<InnerDatabase>);

impl Database {
    /// Init a database from path.  
    /// Note that all database objects with same path share the same core.
    /// So you can create multiple database objects. WCDB will manage them automatically.  
    /// Note that WCDB will not generate a sqlite handle until the first operation(lazy initialization).
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Database> {
        let path = path_to_cstring(path.as_ref())?;
        let db = unsafe { libwcdb_sys::WCDBCoreCreateDatabase(path.as_ptr()) };
        Ok(Database(Arc::new(InnerDatabase(db))))
    }

    /// The tag of the database. Default to nil.
    /// Note that core objects with same path share this tag, even they are not the same object.
    pub fn get_tag(&self) -> Option<Tag> {
        let tag = unsafe { libwcdb_sys::WCDBDatabaseGetTag(self.as_ptr()) };
        if tag == 0 {
            None
        } else {
            Some(tag as Tag)
        }
    }

    /// Set the tag.
    pub fn set_tag(&self, tag: Option<Tag>) {
        let tag = tag.unwrap_or(0);
        unsafe { libwcdb_sys::WCDBDatabaseSetTag(self.as_ptr(), tag as i64) };
    }

    /// The path of the related database.
    pub fn get_path(&self) -> String {
        let path = unsafe { libwcdb_sys::WCDBDatabaseGetPath(self.as_ptr()) };
        crate::utils::c_ptr_to_string_opt(path).unwrap_or_default()
    }

    /// Since WCDB is using lazy initialization,
    /// `init(withPath:)`, `init(withFileURL:)` never failed even the database can't open.
    /// So you can call this to check whether the database can be opened.  
    /// Return false if an error occurs during sqlite handle initialization.
    pub fn can_open(&self) -> bool {
        unsafe { libwcdb_sys::WCDBDatabaseCanOpen(self.as_ptr()) }
    }

    /// Check database is already opened.
    pub fn is_opened(&self) -> bool {
        unsafe { libwcdb_sys::WCDBDatabaseIsOpened(self.as_ptr()) }
    }

    /// Close the database.  
    ///     Since Multi-threaded operation is supported in WCDB,
    ///     other operations in different thread can open the closed database.
    ///     So this method can make sure database is closed in the `onClosed` block.
    ///     All other operations will be blocked until this method returns.
    ///
    /// A close operation consists of 4 steps:  
    ///     1. `blockade`, which blocks all other operations.  
    ///     2. `close`, which waits until all sqlite handles return and closes them.  
    ///     3. `onClosed`, which trigger the callback.  
    ///     4. `unblokade`, which unblocks all other opreations.  
    ///
    /// You can simply call `close:` to do all steps above or call these separately.  
    /// Since this method will wait until all sqlite handles return, it may lead to deadlock in some bad practice.
    ///     The key to avoid deadlock is to make sure all WCDB objects in current thread is dealloced. In detail:  
    ///     1. You should not keep WCDB objects, including `Insert`, `Delete`, `Update`, `Select`, `RowSelect`,
    ///        `MultiSelect`, `Handle`, `PreparedStatement`. These objects should not be kept.
    ///        You should get them, use them, then release them right away.  
    ///     2. WCDB objects may not be out of its' scope.  
    ///     The best practice is to call `close:` in sub-thread and display a loading animation in main thread.  
    ///
    ///     //close directly
    ///     database.close(onClosed: { () throws -> Void in
    ///         //do something on this closed database
    ///     })
    ///
    ///     //close separately
    ///     database.blockade()
    ///     database.close()
    ///     //do something on this closed database
    ///     database.unblockade()
    /// Note that panic in callback will lead to crash.
    pub fn close_with_callback<F>(&self, on_closed: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe extern "C" fn on_closed_callback<F>(ptr: *mut std::ffi::c_void)
        where
            F: FnOnce() + Send + 'static,
        {
            let inner = Box::from_raw(ptr as *mut F);
            inner();
        }
        unsafe {
            libwcdb_sys::WCDBDatabaseClose(
                self.as_ptr(),
                Box::into_raw(Box::new(on_closed)) as _,
                Some(on_closed_callback::<F>),
            );
        }
    }

    /// Close the database.
    pub fn close(&self) {
        unsafe { libwcdb_sys::WCDBDatabaseClose(self.as_ptr(), ptr::null_mut(), None) };
    }

    /// Blockade the database.
    pub fn blockade(&self) {
        unsafe { libwcdb_sys::WCDBDatabaseBlockade(self.as_ptr()) };
    }

    /// Unblockade the database.
    pub fn unblockade(&self) {
        unsafe { libwcdb_sys::WCDBDatabaseUnblockade(self.as_ptr()) };
    }

    /// Check whether database is blockaded.
    pub fn is_blockaded(&self) -> bool {
        unsafe { libwcdb_sys::WCDBDatabaseIsBlockaded(self.as_ptr()) }
    }

    /// Purge all unused memory of this database.  
    /// WCDB will cache and reuse some sqlite handles to improve performance.   
    /// The max count of free sqlite handles is same
    /// as the number of concurrent threads supported by the hardware implementation.  
    /// You can call it to save some memory.
    pub fn purge(&self) {
        unsafe { libwcdb_sys::WCDBDatabasePurge(self.as_ptr()) };
    }

    /// Purge all unused memory of all databases.  
    /// Note that WCDB will call this interface automatically while it receives memory warning on iOS.
    pub fn purge_all() {
        unsafe { libwcdb_sys::WCDBCorePurgeAllDatabase() };
    }

    /// Create a `Handle` for current database.
    /// `Handle` is a wrapper for sqlite db handle of type `sqlite3*`,
    /// and the sqlite db handle is lazy initialized and will not be actually generated until the first operation on current handle takes place.
    /// Note that all `Handle` created by the current database in the current thread will share the same sqlite db handle internally,
    /// so it can avoid the deadlock between different sqlite db handles in some extreme cases.
    /// - Parameter writeHint: A hint as to whether the handle will be used to update content in the database. It doesn't need to be precise.
    /// - Returns: A new `Handle`.
    /// - Throws: `Error`
    pub fn get_handle_with_hint(&self, write_hint: bool) -> Result<Handle> {
        let handle = unsafe { libwcdb_sys::WCDBDatabaseGetHandle(self.as_ptr(), write_hint) };
        let handle_valid = unsafe { libwcdb_sys::WCDBHandleCheckValid(handle) };
        if handle_valid {
            Ok(Handle::owned(handle, self.clone()))
        } else {
            Err(self.error())
        }
    }

    /// Create a `Handle` for current database.
    pub fn get_handle(&self) -> Result<Handle> {
        self.get_handle_with_hint(false)
    }

    pub fn error(&self) -> error::Error {
        let err = unsafe { libwcdb_sys::WCDBDatabaseGetError(self.as_ptr()) };
        let wcdb_error = Box::new(WCDBError::from(err));
        wcdb_error.into()
    }

    pub fn as_ptr(&self) -> libwcdb_sys::CPPDatabase {
        self.0 .0
    }
}

// Config

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CipherVersion {
    DefaultVersion = 0,
    Version1 = 1,
    Version2 = 2,
    Version3 = 3,
    Version4 = 4,
}

pub struct CipherConfig<'a> {
    pub key: Option<&'a [u8]>,
    pub page_size: i32,
    pub cipher_version: CipherVersion,
}

impl Default for CipherConfig<'_> {
    fn default() -> Self {
        Self {
            key: None,
            page_size: 4096,
            cipher_version: CipherVersion::DefaultVersion,
        }
    }
}

#[derive(Default)]
pub struct CipherConfigBuilder<'a>(CipherConfig<'a>);

impl<'a> CipherConfigBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key(mut self, key: &'a [u8]) -> Self {
        self.0.key = Some(key);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.0.page_size = page_size;
        self
    }

    pub fn cipher_version(mut self, cipher_version: CipherVersion) -> Self {
        self.0.cipher_version = cipher_version;
        self
    }

    pub fn build(self) -> CipherConfig<'a> {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ConfigPriority {
    Highest = -2147483648, // Only For cipher config
    High = -100,
    Default = 0,
    Low = 100,
}

impl Database {
    pub fn set_cipher(&self, config: CipherConfig) {
        if let Some(key) = config.key {
            let key_size = key.len();
            let key = key.as_ptr();
            let page_size = config.page_size;
            let cipher_version = config.cipher_version as i32;
            unsafe {
                libwcdb_sys::WCDBDatabaseConfigCipher(
                    self.as_ptr(),
                    key,
                    key_size as i32,
                    page_size,
                    cipher_version,
                )
            };
        } else {
            unsafe { libwcdb_sys::WCDBDatabaseConfigCipher(self.as_ptr(), ptr::null(), 0, 0, 0) };
        }
    }

    pub fn set_default_cipher(version: CipherVersion) {
        let cipher_version = version as i32;
        unsafe { libwcdb_sys::WCDBCoreSetDefaultCipherConfig(cipher_version) };
    }

    /// Set config for this database.
    ///
    /// Since WCDB is a multi-handle database, an executing handle will not apply this config immediately.  
    /// Instead, all handles will run this config before its next operation.
    ///
    /// If you want to add cipher config, please use `ConfigPriority.highest`.
    ///
    ///     database.setConfig(named: "demo", withInvocation: { (handle: Handle) throws in
    ///         try handle.exec(StatementPragma().pragma(.secureDelete).to(true))
    ///     }, withUninvocation: { (handle: Handle) throws in
    ///         try handle.exec(StatementPragma().pragma(.secureDelete).to(false))
    ///     }, withPriority: .high)
    ///
    /// - Parameters:
    ///   - name: The Identifier for this config
    ///   - callback: config
    ///   - invocation: The callback will be called when the handle is opened
    ///  - uninvocation: The callback will be called when the handle is closed
    ///   - order: The smaller number is called first
    /// Note that panic in callback will lead to crash.
    pub fn set_config<F1, F2>(
        &self,
        name: &str,
        invocation: F1,
        uninvocation: F2,
        priority: ConfigPriority,
    ) -> Result<()>
    where
        F1: Fn(Handle) -> bool + 'static,
        F2: Fn(Handle) -> bool + 'static,
    {
        let t1 = self.clone();
        let t2 = self.clone();
        self.set_config_inner(
            name,
            invocation,
            uninvocation,
            move || t1.clone(),
            move || t2.clone(),
            priority,
        )
    }
    fn set_config_inner<F1, F2, F3, F4>(
        &self,
        name: &str,
        invocation: F1,
        uninvocation: F2,
        get_self_invocation: F3,
        get_self_uninvocation: F4,
        priority: ConfigPriority,
    ) -> Result<()>
    where
        F1: Fn(Handle) -> bool + 'static,
        F2: Fn(Handle) -> bool + 'static,
        F3: Fn() -> Database + 'static,
        F4: Fn() -> Database + 'static,
    {
        struct Context<F1, F2> {
            call: F1,
            get_self: F2,
        }

        type CallContext<F1, F2, F3, F4> = Either<Context<F1, F3>, Context<F2, F4>>;

        unsafe extern "C" fn callback_invocation<F1, F2, F3, F4>(
            context: *mut std::ffi::c_void,
            handle: libwcdb_sys::CPPHandle,
        ) -> bool
        where
            F1: Fn(Handle) -> bool + 'static,
            F2: Fn(Handle) -> bool + 'static,
            F3: Fn() -> Database + 'static,
            F4: Fn() -> Database + 'static,
        {
            let raw_context = Box::from_raw(context as *mut CallContext<F1, F2, F3, F4>);
            let ret = raw_context.as_ref().as_ref().left().map_or(true, |c| {
                let handle = Handle::reference(handle, (c.get_self)());
                (c.call)(handle)
            });
            Box::into_raw(raw_context); // just forget it
            ret
        }

        unsafe extern "C" fn callback_uninvocation<F1, F2, F3, F4>(
            context: *mut std::ffi::c_void,
            handle: libwcdb_sys::CPPHandle,
        ) -> bool
        where
            F1: Fn(Handle) -> bool + 'static,
            F2: Fn(Handle) -> bool + 'static,
            F3: Fn() -> Database + 'static,
            F4: Fn() -> Database + 'static,
        {
            let raw_context = Box::from_raw(context as *mut CallContext<F1, F2, F3, F4>);
            let ret = raw_context.as_ref().as_ref().right().map_or(true, |c| {
                let handle = Handle::reference(handle, (c.get_self)());
                (c.call)(handle)
            });
            Box::into_raw(raw_context); // just forget it
            ret
        }

        unsafe extern "C" fn callback_destructor<F1, F2, F3, F4>(context: *mut std::ffi::c_void)
        where
            F1: Fn(Handle) -> bool + 'static,
            F2: Fn(Handle) -> bool + 'static,
            F3: Fn() -> Database + 'static,
            F4: Fn() -> Database + 'static,
        {
            let _ = Box::from_raw(context as *mut CallContext<F1, F2, F3, F4>);
        }

        let name = CString::new(name)?;
        let invocation: *mut CallContext<F1, F2, F3, F4> =
            Box::into_raw(Box::new(CallContext::Left(Context {
                call: invocation,
                get_self: get_self_invocation,
            })));

        let uninvocation: *mut CallContext<F1, F2, F3, F4> =
            Box::into_raw(Box::new(CallContext::Right(Context {
                call: uninvocation,
                get_self: get_self_uninvocation,
            })));
        unsafe {
            libwcdb_sys::WCDBDatabaseConfig(
                self.as_ptr(),
                name.as_ptr(),
                Some(callback_invocation::<F1, F2, F3, F4>),
                invocation as _,
                Some(callback_uninvocation::<F1, F2, F3, F4>),
                uninvocation as _,
                priority as i32,
                Some(callback_destructor::<F1, F2, F3, F4>),
            );
        }
        Ok(())
    }
}
