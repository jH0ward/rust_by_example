fn traverse_matrix_loops(n_rows: u32, n_cols: u32) {
    let mut row = 0;
    'row_loop: loop {
        if row >= n_rows {
            break 'row_loop;
        }
        let mut col = 0;
        'col_loop: loop {
            if col >= n_cols {
                break 'col_loop;
            }
            println!("Visiting matrix[{row}][{col}]");
            col = col + 1;
        }
        row = row + 1;
    }
    println!("Done loop traversal");
}

fn traverse_matrix_while(n_rows: u32, n_cols: u32) {
    let mut i = 0;
    while i < n_rows {
        let mut j = 0;
        while j < n_cols {
            println!("Visiting matrix[{i}][{j}]");
            j += 1;
        }
        i += 1;
    }
    println!("Done matrix while traversal");
}

fn traverse_matrix_range(n_rows: u32, n_cols: u32) {
    for i in 0..n_rows {
        for j in 0..n_cols {
            println!("Visiting mat[{i}][{j}]");
        }
    }
    println!("Done matrix range traversal");
}

fn main() {
    let condition: bool = false;

    let x: i32 = if condition { 5 } else { 6 };

    println!("{x}");

    let mut i: i32 = 99;

    let done = loop {
        println!("{i} bottles of beer on the wall");
        i = i - 1;
        if i < 0 {
            break "Success";
        }
    };
    println!("{done}");

    // you can label loops and break them by name
    let n_rows = 5;
    let n_columns = 3;
    traverse_matrix_loops(n_rows, n_columns);
    traverse_matrix_while(n_rows, n_columns);

    let ary = [1, 2, 3, 4];

    // can loop over a collection like so
    for x in ary {
        println!("{x}");
    }

    traverse_matrix_range(n_rows, n_columns);
}
