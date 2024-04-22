use crate::utils::cpp_bridged;

cpp_bridged!(pub struct Handle(libwcdb_sys::CPPHandle));
