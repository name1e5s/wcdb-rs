pub trait Statement {
    fn raw_stmt(&self) -> *mut libwcdb_sys::CPPObject;
}
