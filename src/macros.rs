// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


macro_rules! not_init {
    ($val: ident) => {
        if !$val.initialized {
            return Err("TableGen is not initialized".into())
        }
    }
}

macro_rules! not_null {
    ($val: expr) => {
        if $val.is_null() {
            return Err(ErrorKind::NullPtr.into())
        }
    };
    ($val: expr, $ret: expr) => {
        if $val.is_null() {
            Err(ErrorKind::NullPtr.into())
        } else {
            Ok($ret)
        }
    }
}

macro_rules! tg_ffi {
    ($func: expr, $arg1: expr, $ctr: path) => {
        unsafe {
            let val = $func($arg1);
            not_null!(val, $ctr(val))
        }
    };
    ($func: expr, $arg1: expr, $arg2: expr, $ctr: path) => {
        unsafe {
            let val = $func($arg1, $arg2);
            not_null!(val, $ctr(val))
        }
    };
    ($func: expr, $arg1: expr, $arg2: expr, $arg3: expr, $ctr: path) => {
        unsafe {
            let val = $func($arg1, $arg2, $arg3);
            not_null!(val, $ctr(val))
        }
    }
}

macro_rules! tg_ffi_string {
    (freestring, $func: expr, $arg1: expr) => {
        unsafe {
            let cstr = $func($arg1);
            not_null!(cstr);
            let s = Ok(CStr::from_ptr(cstr).to_string_lossy().into_owned());
            TGStringFree(cstr);
            s
        }
    };
    ($func: expr, $arg1: expr) => {
        unsafe {
            let cstr = $func($arg1);
            not_null!(cstr);
            Ok(CStr::from_ptr(cstr).to_string_lossy().into_owned())
        }
    };
    ($func: expr, $arg1: expr, $arg2: expr) => {
        unsafe {
            let cstr = $func($arg1, $arg2);
            not_null!(cstr);
            Ok(CStr::from_ptr(cstr).to_string_lossy().into_owned())
        }
    };

}

macro_rules! match_typed_value {
    ($value:expr, $invalid: path, $( $variant1:path = $func:expr, $variant2: path ),+ ) => {
    match $value {
        $(
            $variant1 => {
            if let Ok(v) = $func {
                $variant2(v)
            } else {
                $invalid
            }
        }
        )+
        _ => $invalid,
        }
    }
}
