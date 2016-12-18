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
use std::os::raw::c_char;
use std::ptr;

use api::*;
use record_keeper::RecordKeeper;
use types::Error;

pub struct TableGen {
    tg_ptr: *const CTableGen,
    initialized: bool,
}

impl TableGen {
    pub fn new(source: &str, includes: Vec<&str>) -> Result<TableGen, Error> {
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
            Err(Error::Other("Could not initialize a TableGen instance"))
        }
    }

    pub fn parse(&mut self) -> Result<bool, Error> {
        unsafe {
            if TGParse(self.tg_ptr) > 0 {
                self.initialized = true;
                Ok(true)
            } else {
                Err(Error::Other("Could not parse the source or dependencies"))
            }
        }
    }

    pub fn record_keeper(&self) -> Result<RecordKeeper, Error> {
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
