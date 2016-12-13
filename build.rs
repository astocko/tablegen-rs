extern crate gcc;

use std::env;
use std::path::Path;
use std::process::Command;
use std::str;

use gcc::Config;

const TGSD: [&'static str; 6] = ["Error.cpp",
                                 "Record.cpp",
                                 "SetTheory.cpp",
                                 "StringMatcher.cpp",
                                 "TGLexer.cpp",
                                 "TGParser.cpp"];

fn llvm_config(cmd: Vec<&str>) -> String {
    let output = Command::new("llvm-config-3.9")
        .args(&["--link-static"])
        .args(&cmd)
        .output()
        .unwrap();

    let output = str::from_utf8(&output.stdout).unwrap().trim();
    String::from(output)
}

fn main() {
    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let version = llvm_config(vec!["--version"]);
    let include_dir = llvm_config(vec!["--includedir"]);
    let lib_dir = llvm_config(vec!["--link-static", "--libdir"]);
    let cpp_flags = llvm_config(vec!["--link-static", "--cppflags"]);
    let cxx_flags = llvm_config(vec!["--link-static", "--cxxflags"]);

    let tg_inc_dir = format!("{}/lib/third-party/llvm-{}/lib", cwd, version);
    let tg_src_dir = format!("{}/TableGen", tg_inc_dir);

    env::set_var("CXXFLAGS", format!("{} {}", cpp_flags, cxx_flags));

    let mut config = gcc::Config::new();
    config.cpp(true);
    config.include(include_dir);
    config.include(tg_inc_dir);

    for f in TGSD.iter() {
        config.file(format!("{}/{}", tg_src_dir, f));
    }


    config.file(format!("{}/lib/TableGenWrapper.cc", cwd));
    config.compile("libtblgen.a");

    println!("cargo:rustc-flags=-l dylib=tinfo");
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=LLVMCore");
    println!("cargo:rustc-link-lib=static=LLVMSupport");
}
