use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use api::*;
use record::Record;


pub struct RecordMap {
    rm_ptr: *const CRecordMap,
}

impl RecordMap {
    pub fn from_ptr(rm: *const CRecordMap) -> RecordMap {
        RecordMap { rm_ptr: rm }
    }

    pub fn get(&self, name: &str) -> Option<Record> {
        let name = CString::new(name).unwrap();
        tg_ffi!(TGRecordMapGet, self.rm_ptr, name.as_ptr(), Record::from_ptr)
    }

    pub fn keys(&self) -> Option<Vec<String>> {
        let mut len: usize = 0;
        let mut cstrs = unsafe { TGRecordMapGetKeys(self.rm_ptr, &mut len) };
        not_null!(cstrs);

        let mut strings: Vec<String> = Vec::new();
        for _ in 0..len {
            let s = unsafe {
                not_null!(*cstrs);
                let cs = CStr::from_ptr(*cstrs).to_string_lossy().into_owned();
                cstrs = cstrs.offset(1);
                cs
            };
            strings.push(s);
        }

        unsafe {
            TGStringArrayFree(cstrs);
        }

        Some(strings)
    }
}
