use std::ffi::CStr;

use libwcdb_sys::CPPCommonTableExpression;

use crate::winq::{convert::AsColumn, identifier::WithRawIdentifier};

use super::{column::Column, identifier};

identifier!(CommonTableExpression<CPPCommonTableExpression>);

impl CommonTableExpression {
    pub fn new(table: &CStr) -> CommonTableExpression {
        let ptr = unsafe { libwcdb_sys::WCDBCommonTableExpressionCreate(table.as_ptr()) };
        CommonTableExpression(ptr.into())
    }

    pub fn column<T: AsColumn>(self, column: T) -> CommonTableExpression {
        let column = column.as_column();
        self.column_inner(column)
    }

    fn column_inner(self, column: Column) -> CommonTableExpression {
        (&self, &column)
            .with_raw(|(t, c)| unsafe { libwcdb_sys::WCDBCommonTableExpressionAddColumn(t, c) });
        self
    }

    // fn as aslect
}
