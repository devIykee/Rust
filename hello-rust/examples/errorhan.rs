#![allow(unused)]

fn main() {
    //  panic!("help theres something wrong");

    let v: Vec<i32> = vec![1, 2, 3];
    // v[90]

    let x = v.get(890);
    match x {
        Some(val) => println!("x is {:?}", val),
        None => println!("None"),
    }

    let x = 900;
    let y = 3;

    let q: Result<i32, String> = if y != 0 {
        Ok(x / y)
    } else {
        Err("Div by 0".to_string())
    };

    match q {
        Ok(val) => println!("{} divided by {} is {}", x, y, val),
        Err(err) => print!("Error: {:?}", err),
    }
}
