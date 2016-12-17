use record::Record;
use compound_value::{DagValue, ListValue};

#[derive(Debug)]
pub enum TypedValue {
    Bit(i8),
    Bits(Vec<i8>),
    Code(String),
    Int(i64),
    String(String),
    List(ListValue),
    Dag(DagValue),
    Record(Record),
    Invalid,
}
