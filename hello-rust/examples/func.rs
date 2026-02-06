#![allow(unused)]

fn sum_return(x: u32, y: u32) -> u32 {
    return x + y; // the return keyword is optional
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // because its an expression not a statement we have to remove the semi-column
}
fn print(s: String) {
    println!("{s}{s}{s}{s}{s}")
}

fn main() {
    let x: i32 = 2;
    let y: i32 = 5;
    let z: i32 = sum(x, y);
    print("😭".to_string());
    println!("the sum of {x} and {y} is {z}");
    let w = "s";
}
