#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

fn main() {
let mut scores: HashMap<String, u32> = HashMap::new();

scores.insert("Solana".to_string(), 1);
scores.insert("angela Yu".to_string(), 24);

println!("{:?}", scores);
}