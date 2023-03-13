fn main()
{
    let number = 8;

    if number < 5
    {
        println!("condition true.");
    }
    else
    {
        println!("condition false.");
    }

    let number = 3;
    if number == 0
    {
        println!("number is equal to zero.");
    }
    else
    {
        println!("number is not equal to zero.");
    }

    let newnumber = 11;

    if newnumber % 4 == 0
    {
        println!("Number is divisible by 4.");
    }
    else if newnumber % 3 == 0
    {
        println!("Number is divisible by 3.");
    }
    else if newnumber % 2 == 0
    {
        println!("Number is divisible by 2.");
    }
    else
    {
        println!("Number is not divisible by 4, 3 or 2.");
    }

    // using if in a let statement
    // both arms of the conditional expression must be compatbile types
    // i32 in this case: 5, 234
    let condition = true;
    let var_a = if condition
    {
        5
    }
    else
    {
        234
    };

    println!("var_a is: {}", var_a);
}
