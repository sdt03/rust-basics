use crate::structs::Rectangle;
use crate::methods::Square;
use crate::patterns::Shape;
mod references;
mod borrowers;
mod structs;
mod methods;
mod patterns;
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
    let circle = Shape::Circle(3.0);
    let square = Shape::Square(10.0);
    println!("{}, {}", crate::patterns::calculate_area(circle), crate::patterns::calculate_area(square));
    let circle = Shape::Circle(3.14);
    println!("{}", crate::patterns::calculate_perimeter(circle));

}

fn sum(a: u32, b: u32) -> u32{
    return a + b;
}

