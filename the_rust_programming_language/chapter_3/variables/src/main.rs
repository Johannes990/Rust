fn main()
{
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;  // x can only be set a second time without explicit declaration if it is mutable
    println!("The value of x is: {}", x) ;   

    const MAX_POINTS: u64 = 1_000_000_000_000_000; // all uppercase letters for constants,
                                                   // underscore between words
    println!("MAX_POINTS: {}", MAX_POINTS);

    let y = 5;      // y = 5
    let y = y + 1;  // y = 5 + 1 = 6
    let y = y * 2;  // y = 6 * 2 = 12
                    // by using the let keyword we can make transformations on the value of y
                    // but still have y be immutable after those transformations have been done

    println!("The value of y is: {}", y);   // only last instance of y is used in the printline macro (y = 12)
                                            // this is called shadowing.
    
    let spaces = "    ";
    let spaces = spaces.len();

    println!("There are {} spaces in variable spaces.", spaces);

    let w: u8 = 250;

    println!("The value of w is: {}", w);
    
    let x_float = 2.3;    // f64 standard
    let y_float: f32 = 4.4; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.6;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remaindes
    let remainder = 43 % 8;

    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    // boolean
    let value = true;
    let second_value: bool = false; // with explicit type annotation

    // chars
    let char_1 = 'f';
    let something = 'Ѭ';

    println!("First char is: {}; second char is: {}", char_1, something);

    let tup: (i32, f64, u8) = (500, 6.4, 1);    // tuple with optional type annotations
    let (e, f, g) = tup;    // destructuring through pattern matching to get single values out of the tuple
    println!("values in the tuple are: {}, {}, {}", e, f, g);

    let newtuple: (i32, f64, u8) = (1232, 0.4322, 33);
    let first = newtuple.0; // destructuring by using the period (.) operator, followed by the index
    let second = newtuple.1;
    let third = newtuple.2;
    println!("values in the tuple are: {}, {}, {}", first, second, third);

    let a = [1, 2, 3, 4, 5];    // array, data type with fixed size, every element has to be of the same type
}