
use crate::traits::{Printer, Add1};

pub struct MyStruct {
    pub pub_i32: i32,
    pub pub_f64: f64,
    pub pub_string: String,
    private_i32: i32,
}

impl MyStruct {
    pub fn new() -> MyStruct {
        MyStruct {
            pub_i32: 1,
            pub_f64: 1.1,
            pub_string: String::from("initialized"),
            private_i32: 7,
        }
    }

    pub fn print_all(&self) {
        println!("{} {} {} {}", self.pub_i32, self.pub_f64, self.pub_string, self.private_i32);
    }
}

impl Printer for MyStruct {
    fn print(&self) -> String {
        let s = String::from("s");
        println!("{}", s);
        s
    }
}

impl Add1 for MyStruct {
    fn add_one(&mut self) {
        self.private_i32 += 1;
    }
}
