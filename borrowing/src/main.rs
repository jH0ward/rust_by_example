// returning values that were moved into functions is tedious so let's borrow (create references)
fn main() {
    println!("Hello, world!");

    let s : String = String::from("Coleman");

    let s_len = calc_length(&s);
    println!("Length of {s} is {s_len}");

    // can't pass s b/c it's not mutable

    // also can't convert s to mutable like this
    let mut s_mut = s;
    // let mut s_mut = s.clone();
    add_smiley(&mut s_mut);

    println!("Added smiley: {s_mut}");

    // Rule 1: you can have either one mutable reference or any number of immutable references
    let _r1 = &mut s_mut;
    // Error cannot borrow s_mut as mutable more than once at a time
    // but if we only print r2 instead of r1 it works
    let r2 = &mut s_mut;
    // println!("{r1}");
    println!("{r2}");

    let ri1 = &s_mut;
    let ri2 = &s_mut;

    // we can have as many immutable as we want
    println!("{ri1} and {ri2}");
    // but if a mutable ref is still in scope, the immutable borrows for ri1 and 2 will fail 
    // println!("{r2}");

    // Rule 2: References must always be valid

    let good_s = no_dangle();
    println!("{good_s}");

}

fn calc_length(some_string: &String) -> usize {
    some_string.len()
    // some_string does not own the value it points to, so it's not dropped
}

// this won't work because &String is not mutable
// fn add_smiley(some_string: &String) {
fn add_smiley(some_string: &mut String) {
    some_string.push_str(" :)");
}

// fn dangle() -> &String {
//     let s = String::from("dangle");
//     println!("{s}");
//     &s
//     // Error the func's return type contains a borrowed value but there is no value to
//     // borrow from (because the owner s goes out of scope)
// }

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
}