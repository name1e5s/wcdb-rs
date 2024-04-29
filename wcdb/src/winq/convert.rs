use std::ffi::CStr;

use super::{
    identifiers::{
        column::Column, column_def::ColumnDef, expression::Expression,
        indexed_column::IndexedColumn, literal_value::LiteralValue, ordering_term::OrderingTerm,
        qualified_table::QualifiedTable, result_column::ResultColumn, schema::Schema,
        table_or_subquery::TableOrSubquery,
    },
    types::{ColumnType, Order},
};

pub trait AsOrderingTerm {
    fn as_order(self) -> OrderingTerm;
    fn order(self, order: Order) -> OrderingTerm;
    fn collate(self, collation_name: &CStr) -> OrderingTerm;
}

pub trait AsColumnDef {
    fn as_def(self, column_type: ColumnType) -> ColumnDef;
}

pub trait AsIndexedColumn {
    fn as_index(self) -> IndexedColumn;
}

pub trait AsOrderedIndexedColumn {
    fn as_ordered_index(self, order: Order) -> IndexedColumn;
}

pub trait AsResultColumn {
    fn as_result_column(self) -> ResultColumn;
}

pub trait AsSchema {
    fn as_schema(self) -> Schema;
}

pub trait AsQualifiedTable {
    fn as_qualified_table(self) -> QualifiedTable;
}

pub trait AsTableOrSubquery {
    fn as_table_or_subquery(self) -> TableOrSubquery;
}

pub trait AsExpression {
    fn as_expression(self) -> Expression;
}

// todo: impl AsResultColumn/AsOrder for AsExpression

pub trait AsColumn {
    fn as_column(self) -> Column;
    fn r#in(self, table: &CStr) -> Column;
    fn of<T: AsSchema>(self, schema: T) -> Column;
}

pub trait AsLiteralValue {
    fn as_literal_value(self) -> LiteralValue;
}
