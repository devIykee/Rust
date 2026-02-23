#![allow(unused)]

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: f32, y: f32) {
        // a method
        self.x = x;
        self.y = y;
    }
}
fn main() {
    let mut p = Point::new(1.0, 2.0);
    p.move_to(2.0, 3.0);
}
