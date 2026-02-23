use std::io;

fn main() {
    let mut input = String::new(); // create a mutable string to store input

    println!("Enter something:");

    io::stdin()
        .read_line(&mut input) // read a line into `input`
        .expect("Failed to read line"); // handle potential errors

    println!("You typed: {}", input.trim()); // `.trim()` removes the newline
    println!("You typed: {}", input.trim());
}