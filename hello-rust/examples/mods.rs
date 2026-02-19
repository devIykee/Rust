#![allow(unused)]

mod my {
    pub fn print() {
        println!("print");
    }

    fn private_print() {
        println!("Private print")
    }
}

fn main() {
    my::print();
    
}
