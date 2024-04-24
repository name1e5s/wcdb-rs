use libwcdb_sys::*;

#[derive(Debug, Clone, Copy)]
pub enum Order {
    Asc,
    Desc,
}

impl Order {
    pub fn c(&self) -> WCDBSyntaxOrder {
        match self {
            Order::Asc => WCDBSyntaxOrder_WCDBSyntaxOrder_Asc,
            Order::Desc => WCDBSyntaxOrder_WCDBSyntaxOrder_Desc,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Conflict {
    Rollback,
    Abort,
    Fail,
    Ignore,
    Replace,
}

impl Conflict {
    pub fn c(&self) -> WCDBSyntaxConflictAction {
        match self {
            Conflict::Rollback => WCDBSyntaxConflictAction_WCDBSyntaxConflictAction_Rollback,
            Conflict::Abort => WCDBSyntaxConflictAction_WCDBSyntaxConflictAction_Abort,
            Conflict::Fail => WCDBSyntaxConflictAction_WCDBSyntaxConflictAction_Fail,
            Conflict::Ignore => WCDBSyntaxConflictAction_WCDBSyntaxConflictAction_Ignore,
            Conflict::Replace => WCDBSyntaxConflictAction_WCDBSyntaxConflictAction_Replace,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColumnType {
    Integer32,
    Integer64,
    Text,
    Float,
    Blob,
    Null,
}

impl ColumnType {
    pub fn c(&self) -> WCDBSyntaxColumnType {
        match self {
            ColumnType::Integer32 => WCDBSyntaxColumnType_WCDBSyntaxColumnType_Integer,
            ColumnType::Integer64 => WCDBSyntaxColumnType_WCDBSyntaxColumnType_Integer,
            ColumnType::Text => WCDBSyntaxColumnType_WCDBSyntaxColumnType_Text,
            ColumnType::Float => WCDBSyntaxColumnType_WCDBSyntaxColumnType_Float,
            ColumnType::Blob => WCDBSyntaxColumnType_WCDBSyntaxColumnType_BLOB,
            ColumnType::Null => WCDBSyntaxColumnType_WCDBSyntaxColumnType_Null,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    Deferred,
    Immediate,
    Exclusive,
}

impl TransactionType {
    pub fn c(&self) -> WCDBSyntaxTransactionType {
        match self {
            TransactionType::Deferred => {
                WCDBSyntaxTransactionType_WCDBSyntaxTransactionType_Deferred
            }
            TransactionType::Immediate => {
                WCDBSyntaxTransactionType_WCDBSyntaxTransactionType_Immediate
            }
            TransactionType::Exclusive => {
                WCDBSyntaxTransactionType_WCDBSyntaxTransactionType_Exclusive
            }
        }
    }
}
