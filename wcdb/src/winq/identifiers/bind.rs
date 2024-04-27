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
    use super::*;

    macro_rules! t {
        ($p:expr, $sql:literal) => {{
            let p = $p;
            assert_eq!(p.description(), $sql);
        }};
    }

    #[test]
    fn test_bind_parameter() {
        t!(BindParameter::new(1), "?1");
        t!(BindParameter::named(c"name"), ":name");
        t!(BindParameter::at(c"name"), "@name");
        t!(BindParameter::colon(c"name"), ":name");
        t!(BindParameter::dollar(c"name"), "$name");

        t!(BindParameter::def(), "?");
        t!(BindParameter::_1(), "?1");
        t!(BindParameter::_2(), "?2");
        t!(BindParameter::_3(), "?3");
        t!(BindParameter::_4(), "?4");
        t!(BindParameter::_5(), "?5");
        t!(BindParameter::_6(), "?6");
        t!(BindParameter::_7(), "?7");
        t!(BindParameter::_8(), "?8");
        t!(BindParameter::_9(), "?9");
        t!(BindParameter::_10(), "?10");
        t!(BindParameter::_11(), "?11");
        t!(BindParameter::_12(), "?12");
        t!(BindParameter::_13(), "?13");
        t!(BindParameter::_14(), "?14");
        t!(BindParameter::_15(), "?15");

        let params = BindParameter::bind_parameters(3);
        assert_eq!(params.len(), 3);
        assert_eq!(params[0].description(), "?1");
        assert_eq!(params[1].description(), "?2");
        assert_eq!(params[2].description(), "?3");
    }
}
