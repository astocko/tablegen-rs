#[macro_use]
mod macros;

mod api;
pub mod tablegen;
pub mod record_keeper;
pub mod record_map;
pub mod record;
pub mod record_value;
pub mod compound_value;
pub mod types;
pub mod typed_init;

pub use tablegen::TableGen;
pub use record_keeper::RecordKeeper;
pub use record_map::RecordMap;
pub use record::Record;
pub use record_value::RecordValue;
pub use compound_value::{DagValue, ListValue};
pub use types::TypedValue;
pub use typed_init::TypedInit;



#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::collections::HashSet;

    fn parse_ins(s: &String) -> String {
        let ins: Vec<&str> = s.split(&['\t', '"'][..]).collect();

        if let Some(&x) = ins.get(1) {
            String::from(x)
        } else {
            String::from("")
        }

    }


    #[test]
    fn it_works() {

        let input = "/home/astocko/dev/public/tablegen-rs/ctablegen/third-party/llvm-3.9.\
                     1/lib/Target/X86/X86.td";
        let includes = vec!["/home/astocko/dev/public/tablegen-rs/ctablegen/third-party/llvm-3.9.\
                             1/lib/Target/X86",
                            "/home/astocko/dev/public/tablegen-rs/ctablegen/third-party/llvm-3.9.\
                             1/include"];

        let mut instructions: HashSet<String> = HashSet::new();

        let mut tg = tablegen::TableGen::new(input, includes).unwrap();
        if let Ok(_) = tg.parse() {
            let defs = tg.record_keeper().unwrap().defs().unwrap();
            let keys = defs.keys().unwrap();

            for k in &keys {
                let rec = &defs.get(k).unwrap();
                if rec.anonymous() == false && !k.contains("anonymous") {
                    for val in rec.values_iter().unwrap() {
                        if val.name.unwrap() == "AsmString" {
                            let tv = val.value;
                            match tv {
                                TypedValue::String(x) => {
                                    instructions.insert(parse_ins(&x));
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }

        let mut instructions: Vec<String> = instructions.into_iter().collect();
        instructions.sort();

        for ins in &instructions {
            println!("{}", ins);
        }


    }
}
