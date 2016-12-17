// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_char;

pub type TGBit = i8;
pub type TGBool = i32;

#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum RecordValueType {
    Bit,
    Bits,
    Code,
    Int,
    String,
    List,
    Dag,
    Record,
    Invalid,
}

pub enum CTableGen {}
pub enum CRecordKeeper {}
pub enum CRecordMap {}
pub enum CRecord {}
pub enum CRecordValue {}
pub enum CTypedInit {}

pub enum CRecordValueIter {}
pub enum CListIterator {}
pub enum CDagIterator {}
pub enum CDagPair {}


extern "C" {
    // CTableGen
    pub fn TGInitialize(source: *const c_char,
                        includes_sz: usize,
                        includes: *const *const c_char)
                        -> *const CTableGen;
    pub fn TGFree(tg: *const CTableGen);
    pub fn TGGetRecordKeeper(tg: *const CTableGen) -> *const CRecordKeeper;

    // TGParser
    pub fn TGParse(tg: *const CTableGen) -> i32;

    // LLVM RecordKeeper
    pub fn TGRecordKeeperGetClasses(rk: *const CRecordKeeper) -> *const CRecordMap;
    pub fn TGRecordKeeperGetDefs(rk: *const CRecordKeeper) -> *const CRecordMap;
    pub fn TGRecordKeeperGetClass(rk: *const CRecordKeeper, name: *const c_char) -> *const CRecord;
    pub fn TGRecordKeeperGetDef(rk: *const CRecordKeeper, name: *const c_char) -> *const CRecord;

    // LLVM RecordMap
    pub fn TGRecordMapGet(rm: *const CRecordMap, name: *const c_char) -> *const CRecord;
    pub fn TGRecordMapGetKeys(rm: *const CRecordMap, len: *mut usize) -> *const *const c_char;

    // // LLVM Record
    pub fn TGRecordGetRecords(record: *const CRecord) -> *const CRecordKeeper;
    pub fn TGRecordGetName(record: *const CRecord) -> *const c_char;
    pub fn TGRecordGetValue(record: *const CRecord, name: *const c_char) -> *const CRecordValue;
    pub fn TGRecordGetFieldType(record: *const CRecord, name: *const c_char) -> RecordValueType;
    pub fn TGRecordGetValuesItr(record: *const CRecord) -> *const CRecordValueIter;
    pub fn TGRecordIsAnonymous(record: *const CRecord) -> TGBool;

    // // LLVM RecordVal
    pub fn TGRecordValGetName(record_val: *const CRecordValue) -> *const c_char;
    pub fn TGRecordValGetType(record_val: *const CRecordValue) -> RecordValueType;
    pub fn TGRecordValGetValAsNewString(record_val: *const CRecordValue) -> *const c_char;
    pub fn TGRecordValGetValAsBit(record_val: *const CRecordValue, bit: *mut i8) -> TGBool;
    pub fn TGRecordValGetValAsBits(record_val: *const CRecordValue, len: *mut usize) -> *mut i8;
    pub fn TGRecordValGetValAsInt(record_val: *const CRecordValue, integer: *mut i64) -> TGBool;
    pub fn TGRecordValGetValAsRecord(record_val: *const CRecordValue) -> *const CRecord;

    // // LLVM RecordVal Iterators
    pub fn TGRecordValItrNext(iter: *const CRecordValueIter) -> *const CRecordValue;

    // LLVM ListType
    pub fn TGListRecordGetType(record_val: *const CRecordValue) -> RecordValueType;
    pub fn TGListRecordGetValues(record_val: *const CRecordValue) -> *const CListIterator;
    pub fn TGListItrNext(iter: *const CListIterator) -> *const CTypedInit;

    // LLVM DagType
    pub fn TGDagRecordGetValues(record_val: *const CRecordValue) -> *const CDagIterator;
    pub fn TGDagItrNext(iter: *const CDagIterator) -> *const CTypedInit;
    pub fn TGDagItrNextPair(iter: *const CDagIterator) -> *const CDagPair;
    pub fn TGDagPairGetKey(pair: *const CDagPair) -> *const c_char;
    pub fn TGDagPairGetValue(pair: *const CDagPair) -> *const CTypedInit;


    // LLVM TypedInit
    pub fn TGInitRecType(init_ref: *const CTypedInit) -> RecordValueType;
    pub fn TGStringInitGetValueNewString(init_ref: *const CTypedInit) -> *const c_char;
    pub fn TGBitInitGetValue(init_ref: *const CTypedInit, bit: *mut i8) -> TGBool;
    pub fn TGBitsInitGetValue(init_ref: *const CTypedInit, len: *mut usize) -> *mut i8;
    pub fn TGIntInitGetValue(init_ref: *const CTypedInit, integer: *mut i64) -> TGBool;
    pub fn TGRecordInitGetValue(init_ref: *const CTypedInit) -> *const CRecord;


    // // Memory
    pub fn TGBitArrayFree(bit_array: *const i8);
    pub fn TGStringFree(string: *const c_char);
    pub fn TGStringArrayFree(strings: *const *const c_char);
    pub fn TGRecordValItrFree(iter: *const CRecordValueIter);
    pub fn TGListItrFree(iter: *const CListIterator);
    pub fn TGDagItrFree(iter: *const CDagIterator);
}
