#![allow(unused)]
mod her {
    pub fn print() {
        println!("her");
    }
}
mod my {
    pub fn print() {
        crate::her::print();   
        println!("print");
    }

    fn private_print() {
        println!("Private print")
    }

    pub mod a {
       pub fn print() {
            println!("a");
        }
    }
}

fn main() {
    my::print();
    my::a::print();
}
