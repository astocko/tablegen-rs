extern crate cmake;

use std::env;
use std::fs::File;
use std::io::Read;

use cmake::Config;


fn main() {
    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();


    let dst = Config::new("ctablegen")
        .define("LLVM_CONFIG_BIN", "llvm-config-3.9")
        .cxxflag("")
        .build();

    let mut cfg = File::open(format!("{}/llvm_lib_dir.cfg", dst.display())).unwrap();
    let mut lib_dir = String::new();
    cfg.read_to_string(&mut lib_dir);

    println!("cargo:rustc-flags=-l dylib=tinfo");
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=LLVMCore");
    println!("cargo:rustc-link-lib=static=LLVMSupport");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=tblgen");


    let target = {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") {
            Some("c++")
        } else if target.contains("freebsd") {
            Some("c++")
        } else {
            Some("stdc++")
        }
    };

    if let Some(lib) = target {
        println!("cargo:rustc-flags=-l dylib={}", lib);
    }
}
