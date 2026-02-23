#![allow(unused)]

fn main() {
    // strings in rust there are multiple ways of calling a string in rust
    let s: String =  String::from("I beleive i can fly"); // String owns the data
    let s2: String = "im sure i can fly".to_string();
    let length:usize = s2.len(); 

    let s3: &str = &s2[..4];

    println!("{} hmm", s3);

    let s4: &str = "cow";

    let s_view: String = format!("{}: {}", s4, s);

    println!("{}", s_view);
}

