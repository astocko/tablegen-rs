macro_rules! not_init {
    ($val: ident) => {
        if !$val.initialized {
            return None
        }
    }
}

macro_rules! not_null {
    ($val: expr) => {
        if $val == ptr::null() {
            return None
        }
    }
}

macro_rules! tg_ffi {
    ($func: expr, $arg1: expr, $ctr: path) => {
        unsafe {
            let val = $func($arg1);
            if val == ptr::null() {
                None
            } else {
                Some($ctr(val))
            }
        }
    };
    ($func: expr, $arg1: expr, $arg2: expr, $ctr: path) => {
        unsafe {
            let val = $func($arg1, $arg2);
            if val == ptr::null() {
                None
            } else {
                Some($ctr(val))
            }
        }
    };
    ($func: expr, $arg1: expr, $arg2: expr, $arg3: expr, $ctr: path) => {
        unsafe {
            let val = $func($arg1, $arg2, $arg3);
            if val == ptr::null() {
                None
            } else {
                Some($ctr(val))
            }
        }
    }
}

macro_rules! tg_ffi_string {
    (freestring, $func: expr, $arg1: expr) => {
        unsafe {
            let cstr = $func($arg1);
            not_null!(cstr);
            let s = Some(CStr::from_ptr(cstr).to_string_lossy().into_owned());
            TGStringFree(cstr);
            s
        }
    };
    ($func: expr, $arg1: expr) => {
        unsafe {
            let cstr = $func($arg1);
            not_null!(cstr);
            Some(CStr::from_ptr(cstr).to_string_lossy().into_owned())
        }
    };
    ($func: expr, $arg1: expr, $arg2: expr) => {
        unsafe {
            let cstr = $func($arg1, $arg2);
            not_null!(cstr);
            Some(CStr::from_ptr(cstr).to_string_lossy().into_owned())
        }
    };

}

macro_rules! match_typed_value {
    ($value:expr, $invalid: path, $( $variant1:path = $func:expr, $variant2: path ),+ ) => {
    match $value {
        $(
            $variant1 => {
            if let Some(v) = $func {
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
