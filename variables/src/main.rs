fn main() {
    let x = 5;
    // x = x + 1; not allowed b/c it's not mutable

    let x = x + 1; // allowed b/c shadow x by using let again

    println!("x now = {x}");

    let mut y: i32 = 4;

    y  = y + 1;

    println!("y now = {val}", val=y);

    // can't shadow a variable by changing it to constant
    const S: &str = "stringy string";

    // introduce a little scope to introduce a new y
    {
        let y = 1492;
        println!("y in inner scope = {y}");
        println!("x in inner scope = {x}");

    }
    println!("y in outer scope = {y}");

    println!("const s = {S}");
}