fn main() {
    let bp_celsius = fahrenheit_to_celsius(212.0);
    println!("B.P. H2O in Celsius = {bp_celsius}");

    let nth_val = nth_fibonacci(10);
    println!("{nth_val}");

    for i in 1..=12 {
        process_day_n(i);
    }
}

fn get_ordinal(n: usize) -> String {
    let ord_names = [
        "None", "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth",
        "Ninth", "Tenth", "Eleventh", "Twelfth",
    ];
    return ord_names[n].to_string();
}

fn fahrenheit_to_celsius(temp_f: f64) -> f64 {
    (temp_f - 32.0) * 5.0 / 9.0
}

fn nth_fibonacci(n: u32) -> u32 {
    if n <= 2 {
        // 1st is 0, 2nd is 1
        n - 1
    } else {
        nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}

fn process_day_n(mut n: u32) {
    let n_us = usize::try_from(n).unwrap();
    println!(
        "On the {ord} day of Christmas, my true love gave to me",
        ord = get_ordinal(n_us)
    );

    let not_day_one: bool = n > 0;

    while n > 0 {
        if not_day_one && n == 1 {
            println!("and {}", nth_day_of_christmas_lyric(n));
        } else {
            println!("{}", nth_day_of_christmas_lyric(n));
        }
        n -= 1;
    }
}

fn nth_day_of_christmas_lyric(n: u32) -> String {
    let lyric = match n {
        1 => "A partridge in a pear tree",
        2 => "2 Turtle Doves",
        3 => "3 French Hens",
        4 => "4 Calling Birds",
        5 => "5 Gold Rings",
        6 => "6 Geese-a-Laying",
        7 => "7 Swans-a-swimming",
        8 => "8 Maids-a-Milking",
        9 => "9 Ladies Dancing",
        10 => "10 lords a leaping",
        11 => "11 Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => panic!(),
    };
    lyric.to_string()
}
