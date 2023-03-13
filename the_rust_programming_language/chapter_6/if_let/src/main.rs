fn main()
{
    let some_u8_value = Some(0u8);
    
    // a match to only do something when we give it a value of 3.
    match some_u8_value
    {
        some(3) => println!("three"),
        _ => (),
    }

    // shorter way to handle case when we want to do something
    // when we have a value of three, or any single value
    if let Some(3) = some_u8_value
    {
        println!("three");
    }
}
