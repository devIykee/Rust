#![allow(unused)]

fn main() {
    let t: (u32, bool, f32) = (1, true, 3.141);

    // nested tuples the fun part i wonder what data type it'll show
    let nt: ((i32, bool, f64, (bool, &str)), (bool, i32)) =
        ((1, true, 2.4, (false, "chicken")), (true, -23));

    // tuple destructuring
    let (unum, bool, _) = t;


    println!("{} and {}", t.1, t.0);
    println!(
        "first nest - [nt.0]: {:?}\n second nest - [nt.0.1]: {:?}\n third nest - [(nt.0).3]: {:?}\n fourth nest - [(nt.0).3.1] {}\n",
        nt.0,
        (nt.0).1,
        (nt.0).3,
        (nt.0).3.1
    );
}
