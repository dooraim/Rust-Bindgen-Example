#![allow( non_upper_case_globals
        , non_camel_case_types
        , non_snake_case
        , dead_code)]

extern crate bindgen;
use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};
use std::thread::sleep;

fn main(){
  // create_libs_mylibs();
  linklibs();
  generate_bindings();
}

fn linklibs(){
  println!("cargo:rustc-link-search=mybind");
  println!("cargo:rustc-link-lib=test");
}

fn generate_bindings(){
  let bindings = bindgen::Builder::default()
        .clang_arg("-I./mybind/include")
        .clang_arg("-I./mybind/include2")
        .header("mybind/test.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("./bindings.rs"))
        .expect("Couldn't write bindings!");
}
