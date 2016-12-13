use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
pub struct CTableGen {
    parser: *const c_void,
    records: *const c_void,
    source_mgr: *const c_void,
}

extern "C" {
    fn tablegen_new(input: *const c_char,
                    include_count: i32,
                    includes: *const *const c_char)
                    -> *const CTableGen;
    fn tablegen_destroy(tg: *const CTableGen);

    fn tablegen_parse_file(tg: *const CTableGen);
}


pub struct TableGen {
    tblgen: *const CTableGen,
}

impl TableGen {
    pub fn new(input: &str, includes: Vec<&str>) -> Result<TableGen, &'static str> {
        let input = CString::new(input).unwrap();

        let includes: Vec<CString> = includes.iter().map(|&i| CString::new(i).unwrap()).collect();
        let includesv: Vec<*const c_char> = includes.into_iter().map(|i| {i.as_ptr()}).collect();

        // let includes: Vec<*const c_char> =
        //     includes.iter().map(|&x| CString::new(x).unwrap().as_ptr()).collect();

        // let tg = unsafe { tablegen_new(input.as_ptr(), includes.len() as i32, includes.as_ptr()) };

        if tg != ptr::null() {
            Ok(TableGen { tblgen: tg })
        } else {
            Err("Could not create a TableGen instance")
        }
    }

    pub fn parse(&self) {
        unsafe {
            tablegen_parse_file(self.tblgen);
        }
    }
}

impl Drop for TableGen {
    fn drop(&mut self) {
        println!("Dropping");
        unsafe {
            tablegen_destroy(self.tblgen);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let input = "/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.\
                     1/lib/Target/X86/X86.td";
        let includes =
            vec!["/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/lib/Target/X86",
                 "/home/astocko/dev/public/tablegen-rs/lib/third-party/llvm-3.9.1/include"];
        let mut tg = TableGen::new(input, includes).unwrap();

        println!("Parsing!");
        tg.parse();
    }
}
