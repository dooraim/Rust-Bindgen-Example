mod bindings {
    include!("../bindings/bindings.rs");
}

fn main() {
    println!("{} - {} = {}", "1", "2", unsafe{bindings::subtract(1,2)});
    println!("{} + {} = {}", "1", "2", unsafe{bindings::add(1,2)});
    println!("{} * {} = {}", "4", "2", unsafe{bindings::multiply(4,2)});
}
