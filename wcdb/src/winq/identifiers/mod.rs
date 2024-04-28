pub mod bind;
pub mod column;
pub mod column_constraint;
pub mod column_def;
pub mod common_table_expression;
pub mod expression;
pub mod foreign_key;
pub mod frame_spec;
pub mod indexed_column;
pub mod join;
pub mod literal_value;
pub mod ordering_term;
pub mod pragma;
pub mod qualified_table;
pub mod raise_function;
pub mod result_column;
pub mod schema;
pub mod table_constraint;
pub mod table_or_subquery;
pub mod upsert;
pub mod window_def;

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

            #[allow(dead_code)]
            pub(crate) fn from_raw(value: $inner) -> Self {
                Self(value.into())
            }
        }

        impl crate::winq::identifier::WithRawIdentifier<$inner> for $name {
            fn with_raw<F, R>(&self, f: F) -> R
            where
                F: FnOnce($inner) -> R,
            {
                self.0.with_raw(f)
            }
        }
    };
}

use identifier;

#[cfg(test)]
mod tests {
    macro_rules! eq_sql {
        ($p:expr, $sql:literal) => {{
            let p = $p;
            assert_eq!(p.description(), $sql);
        }};
    }
    pub(crate) use eq_sql;
}

#[cfg(test)]
use tests::eq_sql;
