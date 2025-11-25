pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

pub fn area(rectangle: &Rectangle) {
    println!("{}", rectangle.height * rectangle.width);
}