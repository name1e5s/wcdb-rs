use std::{ffi::CStr, os::raw::c_void, ptr};

use libwcdb_sys::CPPColumn;

use super::{identifier, result_column::ResultColumn, schema::Schema};
use crate::winq::{
    convert::{AsColumn, AsSchema},
    identifier::WithRawIdentifier,
};

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

impl AsColumn for Column {
    fn as_column(self) -> Column {
        self
    }

    fn r#in(self, name: &CStr) -> Column {
        self.r#in(name)
    }

    fn of<T: AsSchema>(self, schema: T) -> Column {
        self.of(schema)
    }
}

#[cfg(test)]
mod tests {
    use super::super::eq_sql;
    use super::*;

    #[test]
    fn test_column() {
        eq_sql!(Column::new(c"name"), "name");
        eq_sql!(Column::new_with_table_binding(c"name", ptr::null()), "name");
        eq_sql!(Column::all(), "*");
        eq_sql!(Column::rowid(), "rowid");
        eq_sql!(Column::new(c"name").r#in(c"table"), "table.name");
        eq_sql!(Column::new(c"name").r#as(c"alias"), "name AS alias");
        eq_sql!(
            Column::new(c"name").r#in(c"t").r#as(c"alias"),
            "t.name AS alias"
        );
    }
}
