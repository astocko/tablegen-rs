// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;

use errors::*;

use api::*;
use record::Record;
use compound_value::{DagValue, ListValue};
use types::TypedValue;

pub struct RecordValue {
    rv_ptr: *const CRecordValue,
    pub name: Result<String>,
    pub value: TypedValue,
    value_type: RecordValueType,
}

impl RecordValue {
    pub fn from_ptr(val: *const CRecordValue) -> RecordValue {
        let mut rec_val = RecordValue {
            rv_ptr: val,
            name: Err(ErrorKind::NullPtr.into()),
            value: TypedValue::Invalid,
            value_type: RecordValueType::Invalid,
        };

        rec_val.name = rec_val.name();
        rec_val.value_type = rec_val.val_type();
        rec_val.load_value();
        rec_val
    }

    fn name(&self) -> Result<String> {
        tg_ffi_string!(TGRecordValGetName, self.rv_ptr)
    }

    fn val_type(&self) -> RecordValueType {
        unsafe { TGRecordValGetType(self.rv_ptr) }
    }

    pub fn list_record_type(&self) -> Result<RecordValueType> {
        if self.value_type == RecordValueType::List {
            unsafe {
                Ok(TGListRecordGetType(self.rv_ptr))
            }
        } else {
            Err(ErrorKind::NullPtr.into())
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn load_value(&mut self) {
        let value = match_typed_value!(self.value_type, TypedValue::Invalid,
                                       RecordValueType::Bit = self.get_value_as_bit(), TypedValue::Bit,
                                       RecordValueType::Bits = self.get_value_as_bits(), TypedValue::Bits,
                                       RecordValueType::Code = self.as_string(), TypedValue::Code,
                                       RecordValueType::Int = self.get_value_as_int(), TypedValue::Int,
                                       RecordValueType::String = self.as_string(), TypedValue::String,
                                       RecordValueType::List = self.get_value_as_list(), TypedValue::List,
                                       RecordValueType::Dag = self.get_value_as_dag(), TypedValue::Dag,
                                       RecordValueType::Record = self.get_value_as_record(), TypedValue::Record
        );
        self.value = value;
    }

    fn get_value_as_bit(&self) -> Result<i8> {
        let mut bit: TGBit = -1;
        unsafe {
            TGRecordValGetValAsBit(self.rv_ptr, &mut bit);
        };

        if bit == 0 || bit == 1 {
            Ok(bit)
        } else {
            Err(ErrorKind::InvalidBitRange.into())
        }
    }

    fn get_value_as_bits(&self) -> Result<Vec<i8>> {
        let mut bits: Vec<TGBit> = Vec::new();
        let mut len: usize = 0;
        unsafe {
            let cbits = TGRecordValGetValAsBits(self.rv_ptr, &mut len);
            let mut bits_ptr = cbits;
            for _ in 0..len {
                bits.push(*bits_ptr);
                bits_ptr = bits_ptr.offset(1);
            }
            TGBitArrayFree(cbits);
        }
        if bits.is_empty() { Err(ErrorKind::NullPtr.into()) } else { Ok(bits) }
    }

    fn get_value_as_int(&self) -> Result<i64> {
        let mut int: i64 = 0;
        unsafe {
            TGRecordValGetValAsInt(self.rv_ptr, &mut int);
        };
        Ok(int)
    }

    fn get_value_as_record(&self) -> Result<Record> {
        tg_ffi!(TGRecordValGetValAsRecord, self.rv_ptr, Record::from_ptr)
    }

    pub fn as_string(&self) -> Result<String> {
        tg_ffi_string!(freestring, TGRecordValGetValAsNewString, self.rv_ptr)
    }

    pub fn get_value_as_list(&self) -> Result<ListValue> {
        let name = if let Ok(ref name) = self.name {
            name.clone()
        } else {
            "".into()
        };
        Ok(ListValue::from_ptr(self.rv_ptr, Ok(name)))
    }

    pub fn get_value_as_dag(&self) -> Result<DagValue> {
        let name = if let Ok(ref name) = self.name {
            name.clone()
        } else {
            "".into()
        };
        Ok(DagValue::from_ptr(self.rv_ptr, Ok(name)))
    }

    pub fn value(&self) -> &TypedValue {
        &self.value
    }
}
