#[macro_use]
mod macros;

mod api;
pub mod tablegen;
pub mod record_keeper;
pub mod record_map;
pub mod record;
pub mod record_value;
pub mod compound_value;
pub mod types;
pub mod typed_init;

pub use tablegen::TableGen;
pub use record_keeper::RecordKeeper;
pub use record_map::RecordMap;
pub use record::Record;
pub use record_value::RecordValue;
pub use compound_value::{DagValue, ListValue};
pub use types::TypedValue;
pub use typed_init::TypedInit;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
