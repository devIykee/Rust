#![allow(unused)]

fn main() {
    // arrays in rust

let arr: [u32; 3] = [1, 2, 3];

let arr2: [u32; 10] = [0; 10];

let mut arr3: [i32; 5] = [5, 4, 3, 2, 1];

println!("{:?}\n {:?}\n {:?}\n", arr, arr2, arr3);

// slices

let arr4: [u32; 9] = [2, 4, 5, 6, 7, 3, 9, 8, 1];

let slic: &[u32] = &arr4[..3];

print!("{:?}", slic)
}
