use std::io;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let c_annotated: char = 'C';
    let c_not_annotated = 'C';

    print_type_of(&c_annotated);
    print_type_of(&c_not_annotated);

    let tup = (1, 4, "world");
    println!("{tup:?}");

    let (num1, num2, str1) = tup;

    println!("{num1}, {num2}, {str1}");

    println!("{}", { tup.2 });

    let a1 = [1, 2, 3, 4, 5];

    println!("{a1:?}");

    println!("{}", { a1[2] });

    let _a2 = [0; 5];

    let a3: [u32; 8] = [0, 1, 1, 2, 3, 5, 8, 13];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    let index: usize = index.trim().parse().expect("wtf");

    let result = a3.get(index);

    println!("{result:?}");
}
