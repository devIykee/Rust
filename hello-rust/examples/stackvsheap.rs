#![allow(unused)]

fn main() {
    let x: i32 = 2; // This variable is stored on the heap because it has a fixed size

    let s: String = String::from("Hello SOLANA"); // This variable is stored on the heap because it has the potential for growth

    // we also have the ability to choose to store anything we want on the heap
    let z: Box<u32> = Box::new(3u32); // 10 is stored on the heap while z the pointer is on the stack
}
