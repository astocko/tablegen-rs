use std::ptr;
use std::ffi::CStr;

use api::*;
use record::Record;
use compound_value::{DagValue, ListValue};
use types::TypedValue;

pub struct TypedInit {
    ti_ptr: *const CTypedInit,
}

impl TypedInit {
    pub fn from_ptr(val: *const CTypedInit) -> TypedInit {
        TypedInit { ti_ptr: val }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn to_typed_value(&self) -> TypedValue {
        match_typed_value!(self.get_type(), TypedValue::Invalid,
                                       RecordValueType::Bit = self.get_as_bit(), TypedValue::Bit,
                                       RecordValueType::Bits = self.get_as_bits(), TypedValue::Bits,
                                       RecordValueType::Code = self.get_as_string(), TypedValue::Code,
                                       RecordValueType::Int = self.get_as_int(), TypedValue::Int,
                                       RecordValueType::String = self.get_as_string(), TypedValue::String,
                                       RecordValueType::List = self.get_as_list(), TypedValue::List,
                                       RecordValueType::Dag = self.get_as_dag(), TypedValue::Dag,
                                       RecordValueType::Record = self.get_as_record(), TypedValue::Record
        )
    }

    fn get_type(&self) -> RecordValueType {
        unsafe { TGInitRecType(self.ti_ptr) }
    }

    fn get_as_bit(&self) -> Option<i8> {
        let mut bit: TGBit = -1;
        unsafe {
            TGBitInitGetValue(self.ti_ptr, &mut bit);
        };

        if bit == 0 || bit == 1 {
            Some(bit)
        } else {
            None
        }
    }

    fn get_as_bits(&self) -> Option<Vec<i8>> {
        let mut bits: Vec<TGBit> = Vec::new();
        let mut len: usize = 0;
        unsafe {
            let cbits = TGBitsInitGetValue(self.ti_ptr, &mut len);
            let mut bits_ptr = cbits;
            for _ in 0..len {
                bits.push(*bits_ptr);
                bits_ptr = bits_ptr.offset(1);
            }
            TGBitArrayFree(cbits);
        }
        if bits.is_empty() { None } else { Some(bits) }
    }

    fn get_as_int(&self) -> Option<i64> {
        let mut int: i64 = 0;
        unsafe {
            TGIntInitGetValue(self.ti_ptr, &mut int);
        };
        Some(int)
    }

    fn get_as_record(&self) -> Option<Record> {
        tg_ffi!(TGRecordInitGetValue, self.ti_ptr, Record::from_ptr)
    }

    fn get_as_string(&self) -> Option<String> {
        tg_ffi_string!(freestring, TGStringInitGetValueNewString, self.ti_ptr)
    }

    fn get_as_list(&self) -> Option<ListValue> {
        None
    }

    fn get_as_dag(&self) -> Option<DagValue> {
        None
    }
}
