struct Color(i32, i32, i32);        // these are known as tuple structs used when
struct Point(i32, i32, i32);        // you want to give the whole tuple a name and make
                                    // the tuple be a different type from other tuples

fn main()
{
    let black = Color(0, 0, 0);     // note that black and origin are different types
    let origin = Point(0, 0, 0);    // although the are made of the same types internally
                                    // a function that takes a Point as a parameter can
                                    // not take a Color instead, even though both are
                                    // made of three i32 fields internally.
}
