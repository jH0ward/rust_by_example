fn main() {
    {
        let s = "Hello";
        println!("{s}");
    }
    // s not valid below that scope
    // println!("{s}");

    // now play with String type

    // String type allocates memory on the heap since not known at compile time its size
    let mut s = String::from("Hello");

    println!("{s}");

    s.push_str(" World!");
    println!("{s}");

    // A String has a ptr to memory holding the contents, a length, and a capacity
    // when we assign a new name to s, a move occurs instead of a shallow copy

    let s2 = s;
    // move occurs because String does not implement the Copy trait

    println!("{s2}");

    // below won't compile because we moved the value to s2
    // println!("{s}");

    // we can deep copy with a clone
    let mut s3 = s2.clone();
    s3.push_str(" I'm a clone");
    println!("{s3}");

    println!("Original: {s2}");


    // integers implement the Copy trait and so don't move - they just copy

    let x: u32 = 100;
    let y: u32 = x;

    println!("({x}), ({y})");


    // you can't impelement the copy trait if the type or any of its parts implement Drop
    // some types for Copy -- int, bool, float, char, tuples of the above


    

}
