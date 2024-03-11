use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("Slice element count: {}", slice.len());
}

pub fn test() {
    println!("arr_and_slices:");
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize every element to the same value
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Element count: {}", xs.len());

    println!("Array byte size: {}", mem::size_of_val(&xs));

    // array borrow
    analyze_slice(&xs);

    // partial array borrow as slice
    analyze_slice(&ys[23..320]);

    // accessing array out of bounds is possible
    // to catch that use match {Some, None}
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{i}: {xval}"),
            None => println!("Slow down! {i} is too far!"),
        }
    }

    print_end!();
}