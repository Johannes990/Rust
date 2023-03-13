fn ten() -> i32
{
    10
}

fn plus_one(x: i32) -> i32
{
    x + 1
}

fn main()
{
    println!("Hello world!");
    
    another_function();

    function_with_argument(4000000000);

    function_with_more_arguments(33.0, 236.0);

    let y =
    {
        let s = 44;
        s * 2
    };

    println!("{} = 44 * 2??", y);

    let x = ten();

    println!("x is: {}.", x);

    let variaabel = plus_one(22);

    println!("22 + 1 is: {}.", variaabel);
}

fn another_function()
{
    println!("Another function.");
}

fn function_with_argument(x: u32)
{
    println!("The argument given to the function is: {}.", x);
}

fn function_with_more_arguments(x: f32, y: f32)
{
    println!("The first argument is: {}.", x);
    println!("The second argument is: {}.", y);

    let z = y / x;

    println!("{} divided by {} is {}.", y, x, z);
}
