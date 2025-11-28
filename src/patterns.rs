pub enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32)
}

pub fn calculate_area(s: Shape) -> f32 {
    let area = match s {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    };
    return area;
}

pub fn calculate_perimeter(s: Shape) -> f32 {
    let perimeter = match s {
        Shape::Circle(radius) => 2.0 * 3.14 * radius,
        Shape::Rectangle(width, length ) => 2.0 * (width + length),
        Shape::Square(side) => 4.0 * side
    };
    return perimeter;
}