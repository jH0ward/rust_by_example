fn main() {
    println!("Hello, world!");
    println!("It's me -- Coleman");
    let x = 99. / 10.0;
    println!("{}", x);

    println!("{} days to go!", 2);

    println!("I'm {0} and you're {1}", "zero", "one");

    println!(
        "{subject} {verb} {object}",
        subject = "Joe",
        verb = "cooks",
        object = "crabs"
    );

    println!("42 Base 10: {}", 42);
    println!("42 Base 2: {:b}", 42);
    println!("42 Base 8: {:o}", 42);
    println!("42 Base 16: {:x}", 42);
    println!("42 Base 16: {:X}", 42);

    println!("{number:>5}", number=5);
    println!("{number:>5}", number=15);
    println!("{number:0>5}", number=15);

    let val_pi = 3.14192;
    let width = 5;
    let precision: usize = 4;

    println!("{val_pi:>width$}");
    println!("{val_pi:.precision$}");
}
