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
use types::TypedValue;
use typed_init::TypedInit;



#[derive(Debug)]
pub struct DagValue {
    dag_ptr: *const CRecordValue,
    pub name: Result<String>,
}

impl DagValue {
    pub fn from_ptr(val: *const CRecordValue, name: Result<String>) -> DagValue {
        DagValue {
            dag_ptr: val,
            name: name,
        }
    }

    pub fn values_iter(&self) -> Result<DagIterator> {
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
        let dp_ref = unsafe { TGDagItrNextPair(self.iter) };
        let ti: Result<TypedInit> = tg_ffi!(TGDagPairGetValue, dp_ref, TypedInit::from_ptr);

        let dp = unsafe {
            let name_ptr = TGDagPairGetKey(dp_ref);
            let name = {
                if name_ptr.is_null() {
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

impl Drop for DagIterator {
    fn drop(&mut self) {
        unsafe { TGDagItrFree(self.iter) }
    }
}

#[derive(Debug)]
pub struct ListValue {
    list_ptr: *const CRecordValue,
    pub name: Result<String>,
}

impl ListValue {
    pub fn from_ptr(val: *const CRecordValue, name: Result<String>) -> ListValue {
        ListValue {
            list_ptr: val,
            name: name,
        }
    }

    pub fn values_iter(&self) -> Result<ListIterator> {
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
        let li: Result<TypedInit> = tg_ffi!(TGListItrNext, self.iter, TypedInit::from_ptr);
        if let Ok(li) = li {
            Some(li.to_typed_value())
        } else {
            None
        }
    }
}

impl Drop for ListIterator {
    fn drop(&mut self) {
        unsafe { TGListItrFree(self.iter) }
    }
}
