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

#[cfg(test)]
mod tests {
    use super::super::eq_sql;
    use super::*;

    #[test]
    fn test_bind_parameter() {
        eq_sql!(BindParameter::new(1), "?1");
        eq_sql!(BindParameter::named(c"name"), ":name");
        eq_sql!(BindParameter::at(c"name"), "@name");
        eq_sql!(BindParameter::colon(c"name"), ":name");
        eq_sql!(BindParameter::dollar(c"name"), "$name");

        eq_sql!(BindParameter::def(), "?");
        eq_sql!(BindParameter::_1(), "?1");
        eq_sql!(BindParameter::_2(), "?2");
        eq_sql!(BindParameter::_3(), "?3");
        eq_sql!(BindParameter::_4(), "?4");
        eq_sql!(BindParameter::_5(), "?5");
        eq_sql!(BindParameter::_6(), "?6");
        eq_sql!(BindParameter::_7(), "?7");
        eq_sql!(BindParameter::_8(), "?8");
        eq_sql!(BindParameter::_9(), "?9");
        eq_sql!(BindParameter::_10(), "?10");
        eq_sql!(BindParameter::_11(), "?11");
        eq_sql!(BindParameter::_12(), "?12");
        eq_sql!(BindParameter::_13(), "?13");
        eq_sql!(BindParameter::_14(), "?14");
        eq_sql!(BindParameter::_15(), "?15");

        let params = BindParameter::bind_parameters(3);
        assert_eq!(params.len(), 3);
        assert_eq!(params[0].description(), "?1");
        assert_eq!(params[1].description(), "?2");
        assert_eq!(params[2].description(), "?3");
    }
}
