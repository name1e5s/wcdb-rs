use num_enum::TryFromPrimitive;
use std::{error, ffi::NulError, fmt, result};
use thiserror::Error;

use crate::{utils::c_ptr_to_string_opt, Tag};

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to get a valid C string, {0}")]
    NulError(#[from] NulError),
    #[error("WCDB internal error: {0}")]
    WCDBError(#[from] Box<WCDBError>),
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum WCDBErrorCode {
    OK = 0,
    Error = 1,
    Internal = 2,
    Permission = 3,
    Abort = 4,
    Busy = 5,
    Locked = 6,
    NoMemory = 7,
    Readonly = 8,
    Interrupt = 9,
    IOError = 10,
    Corrupt = 11,
    NotFound = 12,
    Full = 13,
    CantOpen = 14,
    Protocol = 15,
    Empty = 16,
    Schema = 17,
    Exceed = 18,
    Constraint = 19,
    Mismatch = 20,
    Misuse = 21,
    NoLargeFileSupport = 22,
    Authorization = 23,
    Format = 24,
    Range = 25,
    NotADatabase = 26,
    Notice = 27,
    Warning = 28,
    Row = 100,
    Done = 101,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum WCDBExtendErrorCode {
    ErrorMissingCollseq = 257,       // Code.Error | (1 << 8)
    ErrorRetry = 513,                // Code.Error | (2 << 8)
    ErrorSnapshot = 769,             // Code.Error | (3 << 8)
    IOErrorRead = 266,               // Code.IOError | (1 << 8)
    IOErrorShortRead = 522,          // Code.IOError | (2 << 8)
    IOErrorWrite = 778,              // Code.IOError | (3 << 8)
    IOErrorFsync = 1034,             // Code.IOError | (4 << 8)
    IOErrorDirFsync = 1290,          // Code.IOError | (5 << 8)
    IOErrorTruncate = 1546,          // Code.IOError | (6 << 8)
    IOErrorFstat = 1802,             // Code.IOError | (7 << 8)
    IOErrorUnlock = 2058,            // Code.IOError | (8 << 8)
    IOErrorRdlock = 2314,            // Code.IOError | (9 << 8)
    IOErrorDelete = 2570,            // Code.IOError | (10 << 8)
    IOErrorBlocked = 2826,           // Code.IOError | (11 << 8)
    IOErrorNoMemory = 3082,          // Code.IOError | (12 << 8)
    IOErrorAccess = 3338,            // Code.IOError | (13 << 8)
    IOErrorCheckReservedLock = 3594, // Code.IOError | (14 << 8)
    IOErrorLock = 3850,              // Code.IOError | (15 << 8)
    IOErrorClose = 4106,             // Code.IOError | (16 << 8)
    IOErrorDirClose = 4362,          // Code.IOError | (17 << 8)
    IOErrorShmOpen = 4618,           // Code.IOError | (18 << 8)
    IOErrorShmSize = 4874,           // Code.IOError | (19 << 8)
    IOErrorShmLock = 5130,           // Code.IOError | (20 << 8)
    IOErrorShmMap = 5386,            // Code.IOError | (21 << 8)
    IOErrorSeek = 5642,              // Code.IOError | (22 << 8)
    IOErrorDeleteNoEntry = 5898,     // Code.IOError | (23 << 8)
    IOErrorMmap = 6154,              // Code.IOError | (24 << 8)
    IOErrorGetTempPath = 6410,       // Code.IOError | (25 << 8)
    IOErrorConvPath = 6666,          // Code.IOError | (26 << 8)
    IOErrorVnode = 6922,             // Code.IOError | (27 << 8)
    IOErrorAuthorization = 7178,     // Code.IOError | (28 << 8)
    IOErrorBeginAtomic = 7434,       // Code.IOError | (29 << 8)
    IOErrorCommitAtomic = 7690,      // Code.IOError | (30 << 8)
    IOErrorRollbackAtomic = 7946,    // Code.IOError | (31 << 8)
    LockedSharedCache = 262,         // Code.Locked | (1 << 8)
    LockedVirtualTable = 518,        // Code.Locked | (2 << 8)
    BusyRecovery = 261,              // Code.Busy | (1 << 8)
    BusySnapshot = 517,              // Code.Busy | (2 << 8)
    CantOpenNoTempDir = 270,         // Code.CantOpen | (1 << 8)
    CantOpenIsDir = 526,             // Code.CantOpen | (2 << 8)
    CantOpenFullPath = 782,          // Code.CantOpen | (3 << 8)
    CantOpenConvPath = 1038,         // Code.CantOpen | (4 << 8)
    CantOpenDirtyWal = 1294,         // Code.CantOpen | (5 << 8)
    CorruptVirtualTable = 267,       // Code.Corrupt | (1 << 8)
    CorruptSequence = 523,           // Code.Corrupt | (2 << 8)
    ReadonlyRecovery = 264,          // Code.Readonly | (1 << 8)
    ReadonlyCantLock = 520,          // Code.Readonly | (2 << 8)
    ReadonlyRollback = 776,          // Code.Readonly | (3 << 8)
    ReadonlyDatabaseMoved = 1032,    // Code.Readonly | (4 << 8)
    ReadonlyCantInit = 1288,         // Code.Readonly | (5 << 8)
    ReadonlyDirectory = 1544,        // Code.Readonly | (6 << 8)
    AbortRollback = 516,             // Code.Abort | (2 << 8)
    ConstraintCheck = 275,           // Code.Constraint | (1 << 8)
    ConstraintCommitHook = 531,      // Code.Constraint | (2 << 8)
    ConstraintForeignKey = 787,      // Code.Constraint | (3 << 8)
    ConstraintFunction = 1043,       // Code.Constraint | (4 << 8)
    ConstraintNotNull = 1299,        // Code.Constraint | (5 << 8)
    ConstraintPrimaryKey = 1555,     // Code.Constraint | (6 << 8)
    ConstraintTrigger = 1811,        // Code.Constraint | (7 << 8)
    ConstraintUnique = 2067,         // Code.Constraint | (8 << 8)
    ConstraintVirtualTable = 2323,   // Code.Constraint | (9 << 8)
    ConstraintRowID = 2579,          // Code.Constraint | (10 << 8)
    NoticeRecoverWal = 283,          // Code.Notice | (1 << 8)
    NoticeRecoverRollback = 539,     // Code.Notice | (2 << 8)
    WarningAutoIndex = 284,          // Code.Warning | (1 << 8)
    AuthorizationUser = 279,         // Code.Authorization | (1 << 8)
    OKLoadPermanently = 256,         // Code.OK | (1 << 8)
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum WCDBErrorLevel {
    Ignore = 1,
    Debug = 2,
    Notice = 3,
    Warning = 4,
    Error = 5, // Only for the errors that will cause api to return false.
    Fatal = 6, // Application should abort.
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum WCDBErrorKey {
    Tag = 1,
    Path = 2,
    Type = 3,
    Source = 4,
    Sql = 5,
    ExtendedCode = 6,
    Message = 7,
    Invalid = 8,
}

impl From<&str> for WCDBErrorKey {
    fn from(s: &str) -> Self {
        match s {
            "Tag" => WCDBErrorKey::Tag,
            "Path" => WCDBErrorKey::Path,
            "Type" => WCDBErrorKey::Type,
            "Source" => WCDBErrorKey::Source,
            "SQL" => WCDBErrorKey::Sql,
            "ExtCode" => WCDBErrorKey::ExtendedCode,
            "Message" => WCDBErrorKey::Message,
            _ => WCDBErrorKey::Invalid,
        }
    }
}

#[derive(Debug)]
pub enum WCDBExtraInfoValue {
    Int(i64),
    Double(f64),
    String(String),
}

#[derive(Debug)]
pub struct WCDBError {
    pub level: WCDBErrorLevel,
    pub code: WCDBErrorCode,
    pub tag: Option<Tag>,
    pub extended_code: Option<WCDBExtendErrorCode>,
    pub message: Option<String>,
    pub source: Option<String>,
    pub sql: Option<String>,
    pub path: Option<String>,
    pub extra_info: Vec<(String, WCDBExtraInfoValue)>,
}

impl Default for WCDBError {
    fn default() -> Self {
        WCDBError {
            level: WCDBErrorLevel::Error,
            code: WCDBErrorCode::OK,
            tag: None,
            extended_code: None,
            message: None,
            sql: None,
            path: None,
            source: None,
            extra_info: Vec::new(),
        }
    }
}

impl fmt::Display for WCDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WCDBError: level: {:?}, code: {:?}, tag: {:?}, extended_code: {:?}, message: {:?}, sql: {:?}, path: {:?}, extra_info: {:?}", self.level, self.code, self.tag, self.extended_code, self.message, self.sql, self.path, self.extra_info)
    }
}

impl error::Error for WCDBError {}

impl From<libwcdb_sys::CPPError> for WCDBError {
    fn from(err: libwcdb_sys::CPPError) -> WCDBError {
        let level = {
            let raw_level = unsafe { libwcdb_sys::WCDBErrorGetLevel(err) } as i32;
            WCDBErrorLevel::try_from(raw_level).unwrap_or(WCDBErrorLevel::Error)
        };
        let code = {
            let raw_code = unsafe { libwcdb_sys::WCDBErrorGetCode(err) } as i32;
            WCDBErrorCode::try_from(raw_code).unwrap_or(WCDBErrorCode::OK)
        };
        let message = {
            let raw_message = unsafe { libwcdb_sys::WCDBErrorGetMsg(err) };
            c_ptr_to_string_opt(raw_message)
        };
        let mut error = WCDBError {
            level,
            code,
            message,
            ..Default::default()
        };

        unsafe extern "C" fn record_error(
            context: *mut ::std::os::raw::c_void,
            key: *const ::std::os::raw::c_char,
            value: libwcdb_sys::CPPCommonValue,
        ) {
            let error = &mut *(context as *mut WCDBError);
            let raw_key = if let Some(key) = c_ptr_to_string_opt(key) {
                key
            } else {
                return;
            };
            let value_int = value.__bindgen_anon_1.intValue;
            let value_double = value.__bindgen_anon_1.doubleValue;
            let value_c_ptr = value.__bindgen_anon_1.intValue as *const ::std::os::raw::c_char;
            let key = WCDBErrorKey::from(raw_key.as_str());
            match key {
                WCDBErrorKey::Tag => {
                    error.tag = Some(value_int as i32);
                }
                WCDBErrorKey::Path => {
                    error.path = c_ptr_to_string_opt(value_c_ptr);
                }
                WCDBErrorKey::Type => {
                    error.code =
                        WCDBErrorCode::try_from(value_int as i32).unwrap_or(WCDBErrorCode::Error)
                }
                WCDBErrorKey::Source => {
                    error.source = c_ptr_to_string_opt(value_c_ptr);
                }
                WCDBErrorKey::Sql => {
                    error.sql = c_ptr_to_string_opt(value_c_ptr);
                }
                WCDBErrorKey::ExtendedCode => {
                    error.extended_code = Some(
                        WCDBExtendErrorCode::try_from(value_int as i32)
                            .unwrap_or(WCDBExtendErrorCode::ErrorMissingCollseq),
                    );
                }
                WCDBErrorKey::Message => {}
                WCDBErrorKey::Invalid => {
                    let value = match value.type_ {
                        libwcdb_sys::WCDBBridgedType_WCDBBridgedType_Int => {
                            WCDBExtraInfoValue::Int(value_int)
                        }
                        libwcdb_sys::WCDBBridgedType_WCDBBridgedType_Double => {
                            WCDBExtraInfoValue::Double(value_double)
                        }
                        libwcdb_sys::WCDBBridgedType_WCDBBridgedType_String => {
                            WCDBExtraInfoValue::String(
                                c_ptr_to_string_opt(value_c_ptr).unwrap_or_default(),
                            )
                        }
                        _ => WCDBExtraInfoValue::String("Unknown".to_string()),
                    };
                    error.extra_info.push((raw_key, value));
                }
            }
        }

        unsafe {
            libwcdb_sys::WCDBErrorEnumerateAllInfo(
                err,
                &mut error as *mut WCDBError as *mut std::os::raw::c_void,
                Some(record_error),
            )
        };
        error
    }
}
