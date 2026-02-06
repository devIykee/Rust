#![allow(unused)]

fn main() {
    // let mut x: u32 = u32::MAX;
    // x+=1;

    let y: Option<u32> = u32::checked_add(u32::MAX, 1);
    println!("checked add: {:?} ", y);
    let z = u32::wrapping_add(u32::MAX, 2);
    println!("Wrapped add: {:?}", z);
}
