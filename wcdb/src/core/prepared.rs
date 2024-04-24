use crate::utils::cpp_bridged;

cpp_bridged!(pub struct PreparedStatement(libwcdb_sys::CPPHandleStatement));
