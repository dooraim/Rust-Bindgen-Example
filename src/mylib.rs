#![allow(
    clippy::all,
    missing_docs,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]

include!(r#"./bindings.rs"#);

impl Default for Libro {
    fn default() -> Self {
        Libro {
            codice: [0; 5],       // Initialize to zero (empty)
            titolo: [0; 41],      // Initialize to zero (empty)
            autore: [0; 31],      // Initialize to zero (empty)
            genere: 0,            // Initialize to zero (null character)
            anno: 0,              // Initialize to zero
            copie: 0,             // Initialize to zero
        }
    }
}

impl Number_Functions {
    unsafe fn numbers_add(n: *mut Numbers) -> ::std::os::raw::c_int {
        (*n).x + (*n).y
    }
    unsafe fn numbers_subtract(n: *mut Numbers) -> ::std::os::raw::c_int{
        (*n).x - (*n).y 
    }
    unsafe fn numbers_multiply(n: *mut Numbers) -> ::std::os::raw::c_int {
        (*n).x * (*n).y
    }
    unsafe fn numbers_divide(n: *mut Numbers) -> ::std::os::raw::c_int {
            (*n).x / (*n).y
    }
}

pub fn print_centered_comment(text: &str) {
    // Total width of the comment line
    let total_width = 76;

    // Calculate padding needed to center the text
    let padding = (total_width - text.len()) / 2;

    // Create the border line and text line
    let border_line = format!("/*{}*/", "-".repeat(total_width));
    let text_line = format!(
        "/*{}{}{}*/",
        " ".repeat(padding),
        text,
        " ".repeat(total_width - text.len() - padding)
    );

    // Print the top border, centered text, and bottom border
    println!("\n{}", border_line);
    println!("{}", text_line);
    println!("{}\n", border_line);
}

pub fn single_print_centered_text(text: &str) {
    // Calculate the total padding needed to center the text
    let width: usize = 74;
    let total_padding = width.saturating_sub(text.len());
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    // Construct the formatted string
    let centered_text = format!(
        "{:padding1$}{}{:padding2$}",
        "",
        text,
        "",
        padding1 = left_padding,
        padding2 = right_padding
    );

    // Print the result with comment markers
    println!("\n/* {} */\n", centered_text.replace(" ", "-"));
}
