// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
pub use types::Error;
pub use typed_init::TypedInit;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
