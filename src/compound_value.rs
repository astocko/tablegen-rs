use std::ptr;
use std::ffi::CStr;

use api::*;
use types::TypedValue;
use typed_init::TypedInit;



#[derive(Debug)]
pub struct DagValue {
    dag_ptr: *const CRecordValue,
    pub name: Option<String>,
}

impl DagValue {
    pub fn from_ptr(val: *const CRecordValue, name: Option<String>) -> DagValue {
        DagValue {
            dag_ptr: val,
            name: name,
        }
    }

    pub fn values_iter(&self) -> Option<DagIterator> {
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
            (None, None) => None,
            (Some(x), None) => Some((x, TypedValue::Invalid)),
            (None, Some(x)) => Some((String::from(""), x.to_typed_value())),
            (Some(x), Some(y)) => Some((x, y.to_typed_value())),
        }

    }
}

#[derive(Debug)]
pub struct ListValue {
    list_ptr: *const CRecordValue,
    pub name: Option<String>,
}

impl ListValue {
    pub fn from_ptr(val: *const CRecordValue, name: Option<String>) -> ListValue {
        ListValue {
            list_ptr: val,
            name: name,
        }
    }

    pub fn values_iter(&self) -> Option<ListIterator> {
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
        if let Some(li) = li {
            Some(li.to_typed_value())
        } else {
            None
        }
    }
}
