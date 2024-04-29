use std::ffi::CStr;

use libwcdb_sys::CPPColumnDef;

use crate::winq::{
    convert::{AsColumn, AsExpression},
    identifier::WithRawIdentifier,
    types::{ColumnType, Conflict, Order},
};

use super::{
    column::Column, column_constraint::ColumnConstraint, foreign_key::ForeignKey, identifier,
};

identifier!(ColumnDef<CPPColumnDef>);

impl ColumnDef {
    pub fn with<T: AsColumn>(column: T) -> ColumnDef {
        let column = column.as_column();
        let ptr = column.with_raw(|c| unsafe { libwcdb_sys::WCDBColumnDefCreateWithoutType(c) });
        ColumnDef(ptr.into())
    }

    pub fn with_type<T: AsColumn>(column: T, type_: ColumnType) -> ColumnDef {
        let column = column.as_column();
        let ptr =
            column.with_raw(|c| unsafe { libwcdb_sys::WCDBColumnDefCreateWithType(c, type_.c()) });
        ColumnDef(ptr.into())
    }

    pub fn named(name: &CStr) -> ColumnDef {
        let column = Column::new(name);
        ColumnDef::with(column)
    }

    pub fn named_with_type(name: &CStr, type_: ColumnType) -> ColumnDef {
        let column = Column::new(name);
        ColumnDef::with_type(column, type_)
    }

    pub fn with_constraint(self, constraint: ColumnConstraint) -> ColumnDef {
        (&self, &constraint)
            .with_raw(|(t, c)| unsafe { libwcdb_sys::WCDBColumnDefConfigConstraint(t, c) });
        self
    }

    pub fn primary_key(self, primary_key: ColumnDefPrimaryKey) -> ColumnDef {
        let constraint = ColumnConstraint::new().primary_key();
        let constraint = if let Some(order) = primary_key.order_by {
            constraint.order(order)
        } else {
            constraint
        };

        let constraint = if primary_key.is_auto_increment.unwrap_or(false) {
            constraint.auto_increment()
        } else {
            constraint
        };

        let constraint = if let Some(conflict) = primary_key.on_conflict {
            constraint.conflict(conflict)
        } else {
            constraint
        };

        self.with_constraint(constraint)
    }

    pub fn default<T: AsExpression>(self, value: T) -> ColumnDef {
        let constraint = ColumnConstraint::new().default(value);
        self.with_constraint(constraint)
    }

    pub fn not_null(self) -> ColumnDef {
        let constraint = ColumnConstraint::new().not_null();
        self.with_constraint(constraint)
    }

    pub fn unique(self) -> ColumnDef {
        let constraint = ColumnConstraint::new().unique();
        self.with_constraint(constraint)
    }

    pub fn foreign_key(self, foreign_key: ForeignKey) -> ColumnDef {
        let constraint = ColumnConstraint::new().foreign_key(foreign_key);
        self.with_constraint(constraint)
    }

    pub fn un_indexed(self) -> ColumnDef {
        let constraint = ColumnConstraint::new().un_indexed();
        self.with_constraint(constraint)
    }
}

pub struct ColumnDefPrimaryKey {
    order_by: Option<Order>,
    is_auto_increment: Option<bool>,
    on_conflict: Option<Conflict>,
}

impl ColumnDefPrimaryKey {
    pub fn new() -> Self {
        Self {
            order_by: None,
            is_auto_increment: None,
            on_conflict: None,
        }
    }

    pub fn order(mut self, order: Order) -> Self {
        self.order_by = Some(order);
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.is_auto_increment = Some(true);
        self
    }

    pub fn on_conflict(mut self, conflict: Conflict) -> Self {
        self.on_conflict = Some(conflict);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::eq_sql;
    use super::*;

    fn new_column_def() -> ColumnDef {
        ColumnDef::named_with_type(c"name", ColumnType::Integer32)
    }

    #[test]
    fn test_column_def() {
        eq_sql!(
            ColumnDef::named_with_type(c"name", ColumnType::Float),
            "name REAL"
        );
        eq_sql!(
            ColumnDef::named_with_type(c"name", ColumnType::Text),
            "name TEXT"
        );
        eq_sql!(
            ColumnDef::named_with_type(c"name", ColumnType::Blob),
            "name BLOB"
        );
        eq_sql!(
            ColumnDef::named_with_type(c"name", ColumnType::Null),
            "name NULL"
        );

        eq_sql!(
            new_column_def().primary_key(ColumnDefPrimaryKey::new()),
            "name INTEGER PRIMARY KEY"
        );
        eq_sql!(
            new_column_def().primary_key(ColumnDefPrimaryKey::new().order(Order::Asc)),
            "name INTEGER PRIMARY KEY ASC"
        );
        eq_sql!(
            new_column_def().primary_key(ColumnDefPrimaryKey::new().auto_increment()),
            "name INTEGER PRIMARY KEY AUTOINCREMENT"
        );
        eq_sql!(
            new_column_def().primary_key(ColumnDefPrimaryKey::new().on_conflict(Conflict::Abort)),
            "name INTEGER PRIMARY KEY ON CONFLICT ABORT"
        );
    }
}
