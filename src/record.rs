// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::ffi::CString;

use api::*;
use record_keeper::RecordKeeper;
use record_value::RecordValue;

use errors::*;

#[derive(Debug)]
pub struct Record {
    r_ptr: *const CRecord,
    pub name: Result<String>,
}

impl Record {
    pub fn from_ptr(rec: *const CRecord) -> Record {
        let mut rec = Record {
            r_ptr: rec,
            name: Err(ErrorKind::NullPtr.into()),
        };
        rec.name = rec.name();
        rec
    }

    fn name(&self) -> Result<String> {
        tg_ffi_string!(TGRecordGetName, self.r_ptr)
    }

    pub fn as_string(&self) -> Result<String> {
        tg_ffi_string!(TGRecordAsNewString, self.r_ptr)
    }

    pub fn records(&self) -> Result<RecordKeeper> {
        tg_ffi!(TGRecordGetRecords, self.r_ptr, RecordKeeper::from_ptr)
    }

    pub fn value(&self, name: &str) -> Result<RecordValue> {
        let name = CString::new(name).unwrap();
        tg_ffi!(TGRecordGetValue,
                self.r_ptr,
                name.as_ptr(),
                RecordValue::from_ptr)
    }

    pub fn get_field_type(&self, name: &str) -> RecordValueType {
        let name = CString::new(name).unwrap();
        unsafe { TGRecordGetFieldType(self.r_ptr, name.as_ptr()) }
    }

    pub fn anonymous(&self) -> bool {
        unsafe { TGRecordIsAnonymous(self.r_ptr) > 0 }
    }

    pub fn values_iter(&self) -> Result<RecordIterator> {
        tg_ffi!(TGRecordGetValuesItr, self.r_ptr, RecordIterator::from_ptr)
    }
}

pub struct RecordIterator {
    iter: *const CRecordValueIter,
}

impl RecordIterator {
    fn from_ptr(rvi: *const CRecordValueIter) -> RecordIterator {
        RecordIterator { iter: rvi }
    }
}

impl Iterator for RecordIterator {
    type Item = RecordValue;

    fn next(&mut self) -> Option<RecordValue> {
        let rec_value: Result<RecordValue> =
            tg_ffi!(TGRecordValItrNext, self.iter, RecordValue::from_ptr);

        if let Ok(res) = rec_value {
            Some(res)
        } else {
            None
        }
    }
}

impl Drop for RecordIterator {
    fn drop(&mut self) {
        unsafe {
            TGRecordValItrFree(self.iter);
        }
    }
}
