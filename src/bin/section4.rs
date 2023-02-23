#[derive(Debug)]
struct Car {
    mpg: i8,
    color: String,
    top_speed: i16,
}

impl Car {
    fn new(mpg: i8, color: String, top_speed: i16) -> Car {
        Car {
            mpg: mpg,
            color: color,
            top_speed: top_speed,
        }
    }

    fn set_mpg(&mut self, mpg: i8) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed;
    }
}

struct Person<'a> {
    name: &'a str, // reference to a string, better than String on memory usage and performance but less efficient if need to modify the string
    age: u8,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: u8) -> Self {
        Person { name, age }
    }

    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

fn main() {
    let mut car = Car::new(20, "red".to_string(), 100);
    println!("car: {:?}", car);
    car.set_mpg(30);
    println!("car: {:?}", car);
    car.set_color("blue".to_string());
    println!("car: {:?}", car);
    car.set_color(String::from("green"));
    println!("car: {:?}", car);
    car.set_top_speed(200);
    println!("car: {:?}", car);

    let mut car2 = Car {
        mpg: 20,
        color: "red".to_string(),
        top_speed: 100,
    };
    println!("car2: {:?}", car2);
    car2.mpg = 30;
    println!("car2: {:?}", car2);
    car2.color = "blue".to_string();
    println!("car2: {:?}", car2);
    car2.set_color(String::from("green"));
    println!("car2: {:?}", car2);
    car2.top_speed = 200;
    println!("car2: {:?}", car2);

    let name = "Alice".to_string();
    let age = 30;
    let person = Person::new(&name, age);
    person.greet();
}