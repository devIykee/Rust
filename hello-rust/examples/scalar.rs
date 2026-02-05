#![allow(unused)]

fn main() {
// signed integers
let a: i32 = -22;

// unsigned integers
let b: u32 = 22;

// floating point numbers
let c: f32 = 0.002;

// Booleans
let d: bool = true;

// characters
let e: char = 'A';

// type conversion

let i: i32 = -1;
let u: u32 = i as u32;

println!("{} as u32 is {}", i, u);
}