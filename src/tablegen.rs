use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use api::*;
use record_keeper::RecordKeeper;

pub struct TableGen {
    tg_ptr: *const CTableGen,
    initialized: bool,
}

impl TableGen {
    pub fn new(source: &str, includes: Vec<&str>) -> Result<TableGen, &'static str> {
        let source = CString::new(source).unwrap();
        let cstrings: Vec<CString> = includes.iter().map(|&i| CString::new(i).unwrap()).collect();
        let includes: Vec<*const c_char> = cstrings.iter().map(|i| i.as_ptr()).collect();
        let tg = unsafe { TGInitialize(source.as_ptr(), includes.len(), includes.as_ptr()) };

        if tg != ptr::null() {
            Ok(TableGen {
                tg_ptr: tg,
                initialized: false,
            })
        } else {
            Err("Could not initialize a TableGen instance")
        }
    }

    pub fn parse(&mut self) -> Result<bool, &'static str> {
        unsafe {
            if TGParse(self.tg_ptr) > 0 {
                self.initialized = true;
                Ok(true)
            } else {
                Err("Could not parse the source or dependencies")
            }
        }
    }

    pub fn record_keeper(&self) -> Option<RecordKeeper> {
        not_init!(self);
        tg_ffi!(TGGetRecordKeeper, self.tg_ptr, RecordKeeper::from_ptr)
    }
}

impl Drop for TableGen {
    fn drop(&mut self) {
        unsafe {
            TGFree(self.tg_ptr);
        }
    }
}
