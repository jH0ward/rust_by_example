use std::vec;
use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[" )?;
        for (i, v) in vec.iter().enumerate() {
            if i > 0 {write!(f, ", ");}
            // write!(f, "{}", v);
            write!(f, "{}: {}", i, v);
        }
        write!(f, "]")
    }
}

// Implement Display for a vector
fn main() {
    println!("Hello, world!");
    let v = vec![4, 5, 6];
    println!("{:?}", v);

    let l = List(v);

    println!("{:?}", l);

    println!("{}", l);
}
