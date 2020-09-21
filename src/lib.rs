
pub mod traits;
pub mod structs;

use traits::{Printer, Add1};

pub fn say_hello() {
    println!("{}", String::from("Hi"));
}

pub fn print(item: &impl Printer) {
    item.print();
}

pub fn print2<T: Printer>(a: T, b: T) {
    a.print();
    b.print();
}

pub fn print_and_add(item: &mut (impl Printer + Add1)) {
    item.print();
    item.add_one();
}

pub fn print_and_add2<T: Printer + Add1>(a: &mut T, b: &mut T) {
    a.print();
    a.add_one();
    b.print();
    b.add_one();
}

pub fn some_function<T, U>(t: &mut T, u: &mut U) -> i32
    where T: Add1 + Printer,
          U: Printer
{
    t.add_one();
    t.print();
    u.print();
    3
}
