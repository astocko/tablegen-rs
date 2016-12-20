// Copyright 2016 Alexander Stocko <as@coder.gg>. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cmake;

use std::env;
use std::fs::File;
use std::io::Read;

use cmake::Config;


fn main() {
    let target = env::var("TARGET").unwrap();

    let llvm_config_path: String = if let Ok(bin) = env::var("LLVM_CONFIG_PATH") {
        bin
    } else {
        String::from("llvm-config")
    };


    let dst = Config::new("ctablegen")
        .define("LLVM_CONFIG_BIN", llvm_config_path)
        .cxxflag("")
        .build();

    let mut cfg = File::open(format!("{}/llvm_lib_dir.cfg", dst.display())).unwrap();
    let mut lib_dir = String::new();
    if let Err(_) = cfg.read_to_string(&mut lib_dir) {
        panic!("ctablegen cmake generated configuration file is missing");
    }
    let lines: Vec<&str> = lib_dir.split("\n").collect();
    for l in lines[1].split(' ') {
        let (_, lib) = l.split_at(2);
        println!("cargo:rustc-flags=-l dylib={}", lib);
    }

    println!("cargo:rustc-link-search=native={}", lines[0]);
    println!("cargo:rustc-link-lib=static=LLVMCore");
    println!("cargo:rustc-link-lib=static=LLVMSupport");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ctablegen");


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
