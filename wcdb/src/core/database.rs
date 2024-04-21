use std::path::Path;

use crate::{
    error::{Result, WCDBError},
    utils::{cpp_bridged, path_to_cstring},
};

cpp_bridged!(pub struct Database(libwcdb_sys::CPPDatabase));

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
        let path = path_to_cstring(path.as_ref())?;
        let db = unsafe { libwcdb_sys::WCDBCoreCreateDatabase(path.as_ptr()) };
        Ok(Database(db))
    }

    pub fn error(self) -> WCDBError {
        let err = unsafe { libwcdb_sys::WCDBDatabaseGetError(self.0) };
        err.into()
    }
}
