use rust_playground::{
    say_hello, structs,
    traits::{Add1, Printer},
};

fn main() {
    println!("Hello");
    say_hello();
    let mut s = structs::MyStruct::new();
    s.print_all();
    s.print();
    s.add_one();
    s.print_all();
    rust_playground::print(&s);
}
