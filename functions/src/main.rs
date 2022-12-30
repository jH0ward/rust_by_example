fn main() {
    let x = 14;
    int_plus_one(x);
    let p1 = double_plus_one(9.0);
    println!("Returned from double plus one is {p1}");
    unit_and_measurement(10, "mi");

    // expressions return values
    let y = {
        let x = 100;
        x + 100
    };

    println!("y = {y}");
}

fn int_plus_one(x: i32) {
    println!("{}", { x + 1 });
}

fn double_plus_one(y: f64) -> f64 {
    println!("{}", { y + 1.0 });
    y + 1.0
}

fn unit_and_measurement(value: i32, label: &str) {
    println!("{value}{label}");
}
