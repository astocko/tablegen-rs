use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use api::*;
use record_keeper::RecordKeeper;
use record_value::RecordValue;

pub struct Record {
    r_ptr: *const CRecord,
    pub name: Option<String>,
}

impl Record {
    pub fn from_ptr(rec: *const CRecord) -> Record {
        let mut rec = Record {
            r_ptr: rec,
            name: None,
        };
        rec.name = rec.name();
        rec
    }

    fn name(&mut self) -> Option<String> {
        tg_ffi_string!(TGRecordGetName, self.r_ptr)
    }

    pub fn records(&self) -> Option<RecordKeeper> {
        tg_ffi!(TGRecordGetRecords, self.r_ptr, RecordKeeper::from_ptr)
    }

    pub fn value(&self, name: &str) -> Option<RecordValue> {
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
        unsafe {
            if TGRecordIsAnonymous(self.r_ptr) > 0 {
                true
            } else {
                false
            }
        }
    }

    pub fn values_iter(&self) -> Option<RecordIterator> {
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
        tg_ffi!(TGRecordValItrNext, self.iter, RecordValue::from_ptr)
    }
}

impl Drop for RecordIterator {
    fn drop(&mut self) {
        unsafe {
            TGRecordValItrFree(self.iter);
        }
    }
}
