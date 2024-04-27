use std::{ffi::CStr, os::raw::c_void, ptr};

use libwcdb_sys::CPPColumn;

use super::identifier;

identifier!(Column<CPPColumn>);

impl Column {
    pub fn new(name: &CStr) -> Column {
        Column::new_with_table_binding(name, ptr::null())
    }

    pub fn new_with_table_binding(name: &CStr, table_binding: *const c_void) -> Column {
        let ptr = unsafe { libwcdb_sys::WCDBColumnCreateWithName(name.as_ptr(), table_binding) };
        Column(ptr.into())
    }

    pub fn all() -> Column {
        let ptr = unsafe { libwcdb_sys::WCDBColumnCreateAll() };
        Column(ptr.into())
    }

    pub fn rowid() -> Column {
        let ptr = unsafe { libwcdb_sys::WCDBColumnCreateRowId() };
        Column(ptr.into())
    }

    pub fn r#in(self, name: &CStr) -> Column {
        unsafe { libwcdb_sys::WCDBColumnInTable(self.as_raw(), name.as_ptr()) }
        self
    }

    // todo: as result column
    // todo: column of schema
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($p:expr, $sql:literal) => {{
            let p = $p;
            assert_eq!(p.description(), $sql);
        }};
    }

    #[test]
    fn test_column() {
        t!(Column::new(c"name"), "name");
        t!(Column::new_with_table_binding(c"name", ptr::null()), "name");
        t!(Column::all(), "*");
        t!(Column::rowid(), "rowid");
        t!(Column::new(c"name").r#in(c"table"), "table.name");
    }
}
