fn main()
{
    let reference_to_nothing = dangle();
}

fn dangle() -> &String  // dangle returns a reference to a String
{
    let s = String::from("Hello");  // s is a new String

    &s      // return a reference to String s
}           // Here, s goes out of scope, and is dropped.
            // Its memory goes away. Danger!
