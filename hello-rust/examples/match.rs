#![allow(unused)]

fn main() {
    let x = 1;
    let x = 3;

    match x {
        1 => println!("{} is one", x),
        3..=10 => println!("It's within the range of 3 - 10"),
        i @ _ => println!("{} is not one", i),
    }

    //option
    //result
    // enum

    let y: Option<i32> = Some(3);
    let y: Option<i32> = None;
    match y {
        Some(val) => println!("Some: {val}"),
        None => println!("None"),
    }

    let z: Result<i32, String> = Ok(33);
    let z: Result<i32, String> = Err("Failed".to_string());
    match z {
        Ok(val) => print!("WE got a good value {}", val),
        Err(err) => println!("Error: {}", err),
    }
    
}
