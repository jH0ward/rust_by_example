fn main() {
    // passing a variable to a function will either move or copy, similar to variables
    println!("Hello, world!");

    let s = String::from("January");

    // s value moves into the function so you can't access after
    take_ownership(s);
    // println!("{s}");

    let x: u32 = 11;
    makes_copy(x);
    // this is ok because a copy is made
    println!("In main scope, x = {x}");

    let s2 = give_ownership();
    println!("{s2}");

    let s3 = take_and_give_ownership(s2);
    println!("{s3}");

    // s2 is not invalid because it was given to the function
    // println!("{s2}");

}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(val: u32){
    println!("{val}");
}

fn give_ownership() -> String {
    let s: String = String::from("give fun string");
    s
}
fn take_and_give_ownership(some_string: String) -> String {
    some_string
}
