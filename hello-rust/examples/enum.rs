#![allow(unused)]

use std::num::NonZero;

#[derive(Debug)]
enum Command {
    Play,
    Pause,
    Forward(u32),
    Backward(u32),
    Resize{height: u32, width: u32}
}
fn main() {
let cmd = Command::Play;
let cmd2 = Command:: Forward(10);
let cmd3: Command = Command::Resize { height: 100, width: 200 };

// OPtion<T> enums
let x: Option<i32> = Some(1);
let x: Option<i32> = None;

// the result enum

// parsing a string to a number
// "100" -> 100
let x: Result<i32, String> = Ok(100);
// "26C?"-> error
let y: Result<i32, String> = Err("Oh noooo it failed".to_string());

println!("{:?} {:?} {:?}", cmd, cmd2, cmd3);
}