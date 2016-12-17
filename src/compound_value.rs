use std::ptr;

use api::*;
use types::TypedValue;
use typed_init::TypedInit;



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
    type Item = TypedValue;

    fn next(&mut self) -> Option<TypedValue> {
        let ti = tg_ffi!(TGDagItrNext, self.iter, TypedInit::from_ptr);
        if let Some(ti) = ti {
            Some(ti.to_typed_value())
        } else {
            None
        }
    }
}

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
