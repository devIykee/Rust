#![allow(unused)]


mod her {
    pub fn print() {
        println!("her");
    }
}
mod my {
    // use super::her;

    // pub fn print() {
    //     her::print();   
    //     println!("print");
    // }

    fn private_print() {
        println!("Private print")
    }

    pub mod a {
        use crate::her;

       pub fn print() {
            println!("a");
        }

    her::print();
    }
}

fn main() {
    my::print();
    my::a::print();
}
