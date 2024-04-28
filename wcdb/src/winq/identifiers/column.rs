use std::{ffi::CStr, os::raw::c_void, ptr};

use libwcdb_sys::CPPColumn;

use super::{identifier, result_column::ResultColumn, schema::Schema};
use crate::winq::{convert::AsSchema, identifier::WithRawIdentifier};

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
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnInTable(t, name.as_ptr()) });
        self
    }

    pub fn r#as(&self, alias: &CStr) -> ResultColumn {
        let ptr =
            self.with_raw(|r| unsafe { libwcdb_sys::WCDBColumnConfigAlias(r, alias.as_ptr()) });
        ResultColumn::from_raw(ptr)
    }

    pub fn of<T: AsSchema>(self, schema: T) -> Column {
        self.of_inner(schema.as_schema())
    }

    fn of_inner(self, schema: Schema) -> Column {
        (&self, &schema).with_raw(|(t, s)| unsafe { libwcdb_sys::WCDBColumnOfSchema(t, s) });
        self
    }
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
        t!(Column::new(c"name").r#as(c"alias"), "name AS alias");
    }
}
