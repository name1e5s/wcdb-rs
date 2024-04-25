use std::ffi::CStr;

use libwcdb_sys::CPPBindParameter;

use super::identifier;

identifier!(BindParameter<CPPBindParameter>);

macro_rules! f {
    (pub fn $name:ident($p:ident : i32) = $f:ident) => {
        pub fn $name($p: i32) -> BindParameter {
            let ptr = unsafe { libwcdb_sys::$f($p) };
            BindParameter(ptr.into())
        }
    };
    (pub fn $name:ident($p:ident : &CStr) = $f:ident) => {
        pub fn $name($p: &CStr) -> BindParameter {
            let ptr = unsafe { libwcdb_sys::$f($p.as_ptr()) };
            BindParameter(ptr.into())
        }
    };

    ($name:ident = $value:literal) => {
        pub fn $name() -> BindParameter {
            BindParameter::new($value)
        }
    
    };
}

impl BindParameter {
    f!(pub fn new(num: i32) = WCDBBindparameterCreateQuestionSignType);
    f!(pub fn named(name: &CStr) = WCDBBindparameterCreateColonSignType);
    f!(pub fn at(name: &CStr) = WCDBBindparameterCreateAtSignType);
    f!(pub fn colon(name: &CStr) = WCDBBindparameterCreateColonSignType);
    f!(pub fn dollar(name: &CStr) = WCDBBindparameterCreateDollarSignType);

    f!(def = 0);
    f!(_1 = 1);
    f!(_2 = 2);
    f!(_3 = 3);
    f!(_4 = 4);
    f!(_5 = 5);
    f!(_6 = 6);
    f!(_7 = 7);
    f!(_8 = 8);
    f!(_9 = 9);
    f!(_10 = 10);
    f!(_11 = 11);
    f!(_12 = 12);
    f!(_13 = 13);
    f!(_14 = 14);
    f!(_15 = 15);

    pub fn bind_parameters(num: i32) -> Vec<BindParameter> {
        (0..num)
            .into_iter()
            .map(|i| BindParameter::new(i + 1))
            .collect()
    }
}
