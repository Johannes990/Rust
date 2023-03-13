fn forever_loop()
{
    loop
    {
        println!("I'm going until your PC breaks...");
    }
}

fn loop_until_10() -> i32
{
    let mut counter = 0;

    let result = loop
    {
        counter += 1;

        if counter == 10
        {
            break counter * 2;
        }
    };

    return result
}

fn while_loop(x: i32)
{
    let mut number = x;

    while number != 0
    {
        println!("{}.", number);

        number = number - 1;
    }
}

fn main()
{
    let var = loop_until_10();

    println!("Variable from looping is: {}", var);

    while_loop(10);
    println!("LIFTOFF!!!");

    // while-looping over an array
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index != 5
    {
        println!("array value at intex {} is a[{}].",index, a[index]);

        index += 1;
    }

    // for-looping over an array
    index = 0;
    for element in a.iter()
    {
        println!("array value at index {} is a[{}].", index, element);

        index += 1;
    }

    // reverse range
    for  number in (1..5).rev()
    {
        println!("{}...", number);
    }
    println!("LIFTOFF!!!!");
}
