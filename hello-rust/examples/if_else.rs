#![allow(unused)]

fn main() {
    let x: i32 = 10;

   let z: i32 = if x > 0 {
        println!("x is greater than 0");
        1
    } else if x < 0 {
        println!("x is less than 0");
        -1
    } else {
        println!("x is zero");
        0
    };

    println!("z: {}", z)
}
