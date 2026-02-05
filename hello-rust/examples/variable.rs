// Attribute metadata for the compiler
#![allow(unused)]

// CONSTANTS

const NUM: i32 = 3;

fn main() {
    let y = 1;
    let mut x = 1;
    x += 1;
    let a: i32 = -12; // signed integer
    let b: u32 = 12; // unsigned integer - only non negtive values
    let c: f32 = 3.14; // float
    let d: char = 'A'; // chars
    let e: bool = true; // bool
    let f: (u32, f32, bool) = (17, 2.3, true); // tuple - an ordered, immmutable finite collection of items
    let g: [u32; 6] = [1, 2, 3, 4, 5, 6]; // Array
    let h = "THE WORLD IS SOLANA"; // immutable string
    let i = String::from("The world is beautiful with SOLANA");

    println!("Iyke firmly believes {2} + {1} = {0}", a, y, a + y);
}
