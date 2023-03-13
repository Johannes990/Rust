fn main()
{
    let v: Vec<i32> = Vec::new();   // creating a new empty vector to hold values of type i32
    let v2 = vec![1, 2, 3];         // creating a new vector containing values
    let mut v3 = Vec::new();

    v3.push(3);
    v3.push(4);
    v3.push(5);
    v3.push(6);

    // accessing elements of a vector
    let third: &i32 = &v3[2];   // referencing
    println!("The third element is {}", third);

    match v3.get(2)
    {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in 0..v3.len()
    {
        let element = &v3[i];
        match v3.get(i)
        {
            Some(element) => println!("The {}-t element is {}", i, element),
            None => println!("No {}-th element!", i),
        }
    }

    let vektor = vec![1, 2, 3, 4, 5];

    let does_not_exist = &vektor[100];      // indexing out of range -> this panics and throws an error
    let does_not_exist = vektor.get(100);   // getting element value out of range again -> this returns None
                                            // which we can then handle
    
}
