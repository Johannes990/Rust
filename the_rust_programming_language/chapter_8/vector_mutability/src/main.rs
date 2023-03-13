fn main()
{
    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = &v[0];  // borrowed an immutable reference

    v.push(6);          // now trying to borrow a mutable reference

    println!("The first element is {}", first);
}
