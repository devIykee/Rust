use std::collections::HashMap;

fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 3];

    let v2: Vec<u32> = v.iter().map(|x: &u32| x + 1).collect();

    println!("vec v2: {:?}", v2);

    let tp: Vec<(&str, u32)> = vec![("a", 1), ("b", 1), ("c", 3)];

    let _v3: Vec<(String, u32)> = tp.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    let v4: HashMap<String, u32> = tp.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();

    // println!("v3 {:?}", v3[2].1);
    println!("v3 {:?}", v4);
}
