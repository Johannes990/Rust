fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];     // immutable vector

    for i in &v {                                   // immutable reference
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];                  // mutable vector

    for i in &mut v {                               // mutable reference
        *i += 50;
        println!("{}", i);
    }
}
