pub mod bind;

macro_rules! identifier {
    ($name:ident<$inner:ident>) => {
        pub struct $name(crate::winq::Identifier<$inner>);

        impl $name {
            pub fn as_ptr(&self) -> *mut libwcdb_sys::CPPObject {
                self.0.as_ptr()
            }

            pub fn description(&self) -> String {
                self.0.description()
            }
        }
    };
}

use identifier;
