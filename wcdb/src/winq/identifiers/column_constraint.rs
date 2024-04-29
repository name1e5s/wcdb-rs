use std::{ffi::CStr, ptr};

use libwcdb_sys::CPPColumnConstraint;

use crate::winq::{
    convert::AsExpression,
    identifier::WithRawIdentifier,
    types::{Conflict, Order},
};

use super::{expression::Expression, foreign_key::ForeignKey, identifier};

identifier!(ColumnConstraint<CPPColumnConstraint>);

impl ColumnConstraint {
    pub fn new() -> ColumnConstraint {
        let ptr = unsafe { libwcdb_sys::WCDBColumnConstraintCreate(ptr::null()) };
        ColumnConstraint(ptr.into())
    }

    pub fn named(name: &CStr) -> ColumnConstraint {
        let ptr = unsafe { libwcdb_sys::WCDBColumnConstraintCreate(name.as_ptr()) };
        ColumnConstraint(ptr.into())
    }

    pub fn primary_key(self) -> ColumnConstraint {
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnConstraintConfigPrimaryKey(t) });
        self
    }

    pub fn order(self, order: Order) -> ColumnConstraint {
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnConstraintConfigOrder(t, order.c()) });
        self
    }

    pub fn conflict(self, conflict: Conflict) -> ColumnConstraint {
        self.with_raw(|t| unsafe {
            libwcdb_sys::WCDBColumnConstraintConfigCoflictAction(t, conflict.c())
        });
        self
    }

    pub fn auto_increment(self) -> ColumnConstraint {
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnConstraintConfigAutoIncrement(t) });
        self
    }

    pub fn not_null(self) -> ColumnConstraint {
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnConstraintConfigNotNull(t) });
        self
    }

    pub fn unique(self) -> ColumnConstraint {
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnConstraintConfigUnique(t) });
        self
    }

    pub fn check(self, expr: Expression) -> ColumnConstraint {
        (&self, &expr)
            .with_raw(|(t, e)| unsafe { libwcdb_sys::WCDBColumnConstraintConfigCheck(t, e) });
        self
    }

    pub fn default<T: AsExpression>(self, expr: T) -> ColumnConstraint {
        self.default_inner(expr.as_expression())
    }

    fn default_inner(self, expr: Expression) -> ColumnConstraint {
        (&self, &expr).with_raw(|(t, e)| unsafe {
            libwcdb_sys::WCDBColumnConstraintConfigDefaultValue(t, e)
        });
        self
    }

    pub fn collate(self, name: &CStr) -> ColumnConstraint {
        self.with_raw(|t| unsafe {
            libwcdb_sys::WCDBColumnConstraintConfigCollation(t, name.as_ptr())
        });
        self
    }

    pub fn foreign_key(self, key: ForeignKey) -> ColumnConstraint {
        (&self, &key)
            .with_raw(|(t, k)| unsafe { libwcdb_sys::WCDBColumnConstraintConfigForeignKey(t, k) });
        self
    }

    pub fn un_indexed(self) -> ColumnConstraint {
        self.with_raw(|t| unsafe { libwcdb_sys::WCDBColumnConstraintConfigUnIndexed(t) });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::eq_sql;
    use super::*;

    #[test]
    fn test_column_constraint() {
        eq_sql!(ColumnConstraint::new(), "");

        eq_sql!(ColumnConstraint::new().primary_key(), "PRIMARY KEY");
        eq_sql!(
            ColumnConstraint::named(c"name").primary_key(),
            "CONSTRAINT name PRIMARY KEY"
        );

        eq_sql!(
            ColumnConstraint::new().primary_key().order(Order::Asc),
            "PRIMARY KEY ASC"
        );
        eq_sql!(
            ColumnConstraint::new()
                .primary_key()
                .conflict(Conflict::Abort),
            "PRIMARY KEY ON CONFLICT ABORT"
        );
        eq_sql!(
            ColumnConstraint::new().primary_key().auto_increment(),
            "PRIMARY KEY AUTOINCREMENT"
        );

        eq_sql!(ColumnConstraint::new().not_null(), "NOT NULL");
        eq_sql!(ColumnConstraint::new().unique(), "UNIQUE");
        eq_sql!(
            ColumnConstraint::named(c"name").collate(c"BINARY"),
            "CONSTRAINT name COLLATE BINARY"
        );
    }
}
