mod bindings {
    include!("../bindings/bindings.rs");
}
use std::ffi::{CString};
use std::os::raw::c_char;

impl Default for bindings::Libro {
    fn default() -> Self {
        bindings::Libro {
            codice: [0; 5],     // Inizializza codice con zeri
            titolo: [0; 41],    // Inizializza titolo con zeri
            autore: [0; 31],    // Inizializza autore con zeri
            genere: 0,          // Inizializza genere a zero (carattere nullo)
            anno: 0,            // Inizializza anno a zero
            copie: 0,           // Inizializza copie a zero
        }
    }
}

fn main() {
    println!("{} - {} = {}", "1", "2", unsafe{bindings::subtract(1,2)});
    println!("{} + {} = {}", "1", "2", unsafe{bindings::add(1,2)});
    println!("{} * {} = {}", "4", "2", unsafe{bindings::multiply(4,2)});

    unsafe {
        let filename = CString::new("/home/dooraim/Develop/Rust/FFI-ABI-C/libc/library/txt-files/libri.txt").unwrap();
        let mut l: [bindings::Libro; bindings::max_num_libri as usize] = [Default::default(); bindings::max_num_libri as usize];
        let dimL = unsafe{bindings::leggiLibri(filename.as_ptr() as *mut c_char, l.as_mut_ptr())};
        println!("Number of book read: {}", dimL);
        for i in 0..dimL as usize {
            let libro = &l[i];
            unsafe{bindings::stampaLibro(*libro)};
        }
    }

}
