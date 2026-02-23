#![allow(unused)]

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);

    let mut v2: Vec<u32> = vec![1u32, 2 , 3];

    println!("{:?} {:?}", v, v2.get(1));
}