#![allow(unused)]

use std::vec;

fn main() {
    let mut i: i32 = 0;
    loop {
        println!("loop: {i}");
        i += 1;
        if i > 5 {
            break;
        }
    }

    let mut i: i32 = 0;
    while i <= 5 {
        println!("while loop: {i}");
        i += 1;
    }

    let mut i: i32 = 0;
    for i in 0..6 {
        println!("for loop: {i}");
    }

    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    let vector: Vec<u32> = vec![1, 2, 3, 4, 5];
    let n: usize = vector.len();
    let m: usize = arr.len();

    for i in 0..n {
        println!("Vector index: {i} Value: {}", arr[i]);
    }

    for i in 0..m {
        println!(" array index {i} value: {}", vector[i]);
    }
}
