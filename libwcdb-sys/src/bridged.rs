//! cpp bridged types
use crate::WCDBBridging::*;

macro_rules! bridged {
    ($name:ident) => {
        impl Into<*mut CPPObject> for $name {
            fn into(self) -> *mut CPPObject {
                self.innerValue
            }
        }
    };
}

bridged!(CPPHandleStatement);
bridged!(CPPBinding);
bridged!(CPPHandle);
bridged!(CPPCancellationSignal);
bridged!(CPPBindParameter);
bridged!(CPPColumn);
bridged!(CPPColumnConstraint);
bridged!(CPPColumnDef);
bridged!(CPPCommonTableExpression);
bridged!(CPPExpression);
bridged!(CPPFilter);
bridged!(CPPForeignKey);
bridged!(CPPFrameSpec);
bridged!(CPPIndexedColumn);
bridged!(CPPJoin);
bridged!(CPPLiteralValue);
bridged!(CPPOrderingTerm);
bridged!(CPPPragma);
bridged!(CPPQualifiedTable);
bridged!(CPPRaiseFunction);
bridged!(CPPResultColumn);
bridged!(CPPSchema);
bridged!(CPPTableConstraint);
bridged!(CPPTableOrSubquery);
bridged!(CPPUpsert);
bridged!(CPPWindowDef);
bridged!(CPPStatementAlterTable);
bridged!(CPPStatementAnalyze);
bridged!(CPPStatementAttach);
bridged!(CPPStatementBegin);
bridged!(CPPStatementCommit);
bridged!(CPPStatementCreateIndex);
bridged!(CPPStatementCreateTable);
bridged!(CPPStatementCreateTrigger);
bridged!(CPPStatementCreateView);
bridged!(CPPStatementCreateVirtualTable);
bridged!(CPPStatementDelete);
bridged!(CPPStatementDetach);
bridged!(CPPStatementDropIndex);
bridged!(CPPStatementDropTable);
bridged!(CPPStatementDropTrigger);
bridged!(CPPStatementDropView);
bridged!(CPPStatementExplain);
bridged!(CPPStatementInsert);
bridged!(CPPStatementPragma);
bridged!(CPPStatementReIndex);
bridged!(CPPStatementRelease);
bridged!(CPPStatementRollback);
bridged!(CPPStatementSavepoint);
bridged!(CPPStatementSelect);
bridged!(CPPStatementUpdate);
bridged!(CPPStatementVacuum);
bridged!(CPPDatabase);
bridged!(CPPError);
