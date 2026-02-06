#![allow(unused)]

use std::i128::MAX;

fn main() {
    // signed integers
    let a: i32 = -22;

    // unsigned integers
    let b: u32 = 22;

    // floating point numbers
    let c: f32 = 0.002;

    // booleans
    let d: bool = true;

    // characters
    let e: char = 'A';

    // type conversion

    let i: i32 = -1;
    let u: u32 = i as u32;

    // MIN and MAX

    let i_max = i32::MAX;
    let u_min: u32 = u32::MIN;

    println!("{} as u32 is {}", i, u);
    println!("the maximum i32 value is {}", i_max);
    println!("the minimum u32 value is {}", u_min);
}
