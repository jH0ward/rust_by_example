fn main() {
    println!("Hello, world!");

    let mut test_s = String::from("Hello world");

    let fwi = first_word_indx_bad(&test_s);
    println!("{}", { &test_s[0..fwi] });

    // we can force an out of bounds panic since our fwi doesn't actually know about test_s state
    test_s.clear();

    // println!("{}", { &test_s[0..fwi] });

    // Slices contain pointers to the start and the slice length
    test_s = String::from("Hello world");

    let fw = first_word(&test_s); // immutable borrow
    println!("{fw}");

    // now let's try to force a panic

    test_s.clear();  // mutable borrow
    // println!("{fw}"); // immutable borrow used again


    // String literals are actually slices and immutable references
    let s = "Hello world";
    let s2 = &s[0..3];
    println !("{s2}");

    // a better first word func just takes a string slice instead of a string
    println!("{}", {first_word_best(&s)});



}

fn first_word_best(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }
    &some_string[..]
}

// Return a string slice instead to capture state
fn first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }
    &some_string[..]
}

fn first_word_indx_bad(some_string: &String) -> usize {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    some_string.len()
}
