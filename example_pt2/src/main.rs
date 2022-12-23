use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(i32);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name} ({age})", name = self.name, age = self.age)
    }
}

#[derive(Debug)]
struct XYCoords {
    x: f64,
    y: f64,
}

impl fmt::Display for XYCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P({x}, {y})", x = self.x, y = self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{r} + {i:.1}i", r = self.real, i = self.imag)
    }
}

fn main() {
    println!("{} months in a year", 12);
    println!("{:?} months in a year", 12);

    println!("{1:?} {0:?} is my name backwards", "Joe", "Brown");
    println!(
        "{0} {1} is my name forwards not-in-debug mode",
        "Joe", "Brown"
    );

    let s = Structure(3);
    println!("{:?}", s);
    println!("{:#?}", s);

    println!("{:?}", Deep(14));

    let name: String = "Eleanor".to_string();
    let age = 4;

    let e = Person { name, age };
    println!("{:#?}", e);

    println!("The Person is {}", e);

    let xy = XYCoords { x: 1.0, y: 2.0 };

    println!("{:#?}", xy);

    println!("{}", xy);

    let z = Complex{real: 3.3, imag: 7.2};
    println!("{:#?}", z);
    println!("{}", z);
}
