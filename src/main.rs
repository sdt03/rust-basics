use crate::structs::Rectangle;
use crate::methods::Square;
mod references;
mod borrowers;
mod structs;
mod methods;
fn main() {
    let a: u32 = 1;
    let b: u32 = 9;
    println!("{}", sum(a, b));
    crate::references::references();
    crate::borrowers::borrowers();
    let rect = Rectangle {
        width: 1,
        height: 4
    };
    crate::structs::area(&rect);

    let square1 = Square {
        side: 4
    };
    println!("Area of square is {}", square1.area());

}

fn sum(a: u32, b: u32) -> u32{
    return a + b;
}

