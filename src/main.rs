
#![allow(
    clippy::all,
    missing_docs,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    dead_code
)]

mod mylib;
use mylib::stampaLibro;
use mylib::print_centered_comment;
use mylib::single_print_centered_text;
use mylib::ordina;
use mylib::Libro;
use mylib::multiply;
use mylib::subtract;
use mylib::max_num_libri;
use mylib::add;
use mylib::leggiLibri;
use std::ffi::{CString, c_char};
mod book;
use book::*;

fn main() {
    
    print_centered_comment("MyBind");
    single_print_centered_text("mybind-subtract");
    println!("{} - {} = {}", "1", "2", unsafe{subtract(1,2)});
    single_print_centered_text("mybind-add");
    println!("{} + {} = {}", "1", "2", unsafe{add(1,2)});
    single_print_centered_text("mybind-multiply");
    println!("{} * {} = {}", "4", "2", unsafe{multiply(4,2)});

    print_centered_comment("Library");
    single_print_centered_text("library-read book");
    let filename = CString::new("mybind/library/libri.txt").unwrap();
    let mut l: [Libro; max_num_libri as usize] = [Default::default(); max_num_libri as usize];
    let dimL = unsafe{leggiLibri(filename.as_ptr() as *mut c_char, l.as_mut_ptr())};
    println!("Number of book read: {}", dimL);
    single_print_centered_text("library-stampaLibro");
    for i in 0..dimL as usize {
        let libro = &l[i];
        unsafe{stampaLibro(*libro)};
    }
    single_print_centered_text("library-sort for year");
    unsafe {ordina(l.as_mut_ptr(), dimL)};
    for i in 0..dimL as usize {
        let libro = &l[i];
        unsafe{stampaLibro(*libro)};
    }

    print_centered_comment("My Library");
    let path_book = "/home/dooraim/Develop/Rust/Rust-Bindgen-Example/mybind/library/libri.txt";
    let books: Vec<Book>= Book::get_books(path_book);
    Book::print_books(&books);

    

}
