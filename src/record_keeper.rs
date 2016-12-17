use std::ffi::CString;
use std::ptr;

use api::*;
use record_map::RecordMap;
use record::Record;


pub struct RecordKeeper {
    rk_ptr: *const CRecordKeeper,
}

impl RecordKeeper {
    pub fn from_ptr(rk: *const CRecordKeeper) -> RecordKeeper {
        RecordKeeper { rk_ptr: rk }
    }

    pub fn classes(&self) -> Option<RecordMap> {
        tg_ffi!(TGRecordKeeperGetClasses, self.rk_ptr, RecordMap::from_ptr)
    }

    pub fn defs(&self) -> Option<RecordMap> {
        tg_ffi!(TGRecordKeeperGetDefs, self.rk_ptr, RecordMap::from_ptr)
    }

    pub fn get_class(&self, name: &str) -> Option<Record> {
        let name = CString::new(name).unwrap();
        tg_ffi!(TGRecordKeeperGetClass,
                self.rk_ptr,
                name.as_ptr(),
                Record::from_ptr)
    }

    pub fn get_def(&self, name: &str) -> Option<Record> {
        let name = CString::new(name).unwrap();
        tg_ffi!(TGRecordKeeperGetDef,
                self.rk_ptr,
                name.as_ptr(),
                Record::from_ptr)
    }
}
