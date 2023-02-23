#[derive(Debug)]
enum Shape {Round, Triangle, Square, Rectangle, Oval, Hexagon, Pentagon, Octagon}

impl Shape {
    fn get_sides(&self) -> u8 {
        match self {
            Shape::Round => 0,
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Rectangle => 4,
            Shape::Oval => 0,
            Shape::Hexagon => 6,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
            _ => 0, // unreachable pattern because all variants are covered
        }
    }
}


fn find_shape(name: &str) -> Option<Shape> {
    match name {
        "Round" => Some(Shape::Round),
        "Triangle" => Some(Shape::Triangle),
        "Square" => Some(Shape::Square),
        "Rectangle" => Some(Shape::Rectangle),
        "Oval" => Some(Shape::Oval),
        "Hexagon" => Some(Shape::Hexagon),
        "Pentagon" => Some(Shape::Pentagon),
        "Octagon" => Some(Shape::Octagon),
        _ => None,
    }
}

fn main() {
    let shape = Shape::Triangle;
    println!("shape: {:?}", shape);
    println!("shape sides: {:?}", shape.get_sides());

    let shape = Shape::Square; // shadowing
    println!("shape: {:?}", shape);
    println!("shape sides: {:?}", shape.get_sides());

    let shape = Shape::Rectangle; // shadowing
    println!("shape: {:?}", shape);
    println!("shape sides: {:?}", shape.get_sides());

    let shape = Shape::Round; // shadowing
    println!("shape: {:?}", shape);
    println!("shape sides: {:?}", shape.get_sides());

    let shape = find_shape("Triangle");
    println!("shape: {:?}", shape);
    println!("shape sides: {:?}", shape.unwrap().get_sides());

    let shape = find_shape("Not Square");
    println!("shape: {:?}", shape);
    // println!("shape sides: {:?}", shape.unwrap().get_sides()); // this will panic because unwrap() will try to access the value of None
}
