#![allow(unused)] // an atribute that applies to the whole crate

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32);

struct Empty;

#[derive(Debug)]
struct Circle {
    radius: u32,
    center: Point,
}

fn main() {
    let p = Point{x: 32, y: 12};
    let p3d: Point3D = Point3D(1, 2, 3);
    let c1: Circle = Circle { radius: 12, center: Point{x: 3, y: 4}};
    println!("{:?} {:?} {:?}", p3d, p3d.0, p3d.1);
    println!("{:?} {:?} {:?}", p, p.x, p.y);
    println!("{:?} {:?} {:?} {:?}", c1, c1.radius, c1.center, c1.center.x );
}
