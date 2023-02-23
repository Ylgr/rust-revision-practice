#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> From<(T, U)> for Point<T, U> {
    fn from(tuple: (T, U)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

impl<T, U> Into<(T, U)> for Point<T, U> {
    fn into(self) -> (T, U) {
        (self.x, self.y)
    }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 5.0,
    };

    let circle = Circle {
        radius: 3.0,
    };
    print_area(&rectangle);
    print_area(&circle);

    let tuple = (3, 5);
    let point = Point::from(tuple);
    println!("{:?}", point);
    let new_tuple: (i32, i32) = point.into();
    println!("{:?}", new_tuple);

    let tuple = ('x', 5);
    let point = Point::from(tuple);
    println!("{:?}", point);
    let new_tuple: (char, i32) = point.into();
    println!("{:?}", new_tuple);
}

