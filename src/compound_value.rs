// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use std::ffi::CStr;

use api::*;
use types::TypedValue;
use types::Error;
use typed_init::TypedInit;



#[derive(Debug)]
pub struct DagValue {
    dag_ptr: *const CRecordValue,
    pub name: Result<String, Error>,
}

impl DagValue {
    pub fn from_ptr(val: *const CRecordValue, name: Result<String, Error>) -> DagValue {
        DagValue {
            dag_ptr: val,
            name: name,
        }
    }

    pub fn values_iter(&self) -> Result<DagIterator, Error> {
        tg_ffi!(TGDagRecordGetValues, self.dag_ptr, DagIterator::from_ptr)
    }
}

pub struct DagIterator {
    iter: *const CDagIterator,
}

impl DagIterator {
    fn from_ptr(di: *const CDagIterator) -> DagIterator {
        DagIterator { iter: di }
    }
}

impl Iterator for DagIterator {
    type Item = (String, TypedValue);

    fn next(&mut self) -> Option<(String, TypedValue)> {
        let dp = unsafe {
            let dp_ref = TGDagItrNextPair(self.iter);
            let name_ptr = TGDagPairGetKey(dp_ref);
            let ti = tg_ffi!(TGDagPairGetValue, dp_ref, TypedInit::from_ptr);
            let name = {
                if name_ptr == ptr::null() {
                    None
                } else {
                    Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
                }
            };
            (name, ti)
        };

        match dp {
            (None, Err(_)) => None,
            (Some(x), Err(_)) => Some((x, TypedValue::Invalid)),
            (None, Ok(x)) => Some((String::from(""), x.to_typed_value())),
            (Some(x), Ok(y)) => Some((x, y.to_typed_value())),
        }

    }
}

#[derive(Debug)]
pub struct ListValue {
    list_ptr: *const CRecordValue,
    pub name: Result<String, Error>,
}

impl ListValue {
    pub fn from_ptr(val: *const CRecordValue, name: Result<String, Error>) -> ListValue {
        ListValue {
            list_ptr: val,
            name: name,
        }
    }

    pub fn values_iter(&self) -> Result<ListIterator, Error> {
        tg_ffi!(TGListRecordGetValues, self.list_ptr, ListIterator::from_ptr)
    }
}

pub struct ListIterator {
    iter: *const CListIterator,
}

impl ListIterator {
    fn from_ptr(di: *const CListIterator) -> ListIterator {
        ListIterator { iter: di }
    }
}

impl Iterator for ListIterator {
    type Item = TypedValue;

    fn next(&mut self) -> Option<TypedValue> {
        let li = tg_ffi!(TGListItrNext, self.iter, TypedInit::from_ptr);
        if let Ok(li) = li {
            Some(li.to_typed_value())
        } else {
            None
        }
    }
}
