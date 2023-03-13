enum IpAddrKind     // enum for IP versions 4 and 6
{
    V4,
    V6,
}

struct IpAddr       // struct to represent IP addresses, type and address data fields
{
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind)
{

}

enum ConciseIpAddrKind  // enum for IP versions 4, 6 in a more concise way
{                       // addres data type already noted as u8 and String.
    V4(u8, u8, u8, u8), // No need for structs.
    V6(String),
}

enum Message            // enum type with different data types and objects in composition
{
    Quit,                       // no data type at all
    Move { x: i32, y: i32 },    // anonymous struct of 2 i32 values
    Write(String),              // String
    Color(i32, i32, i32),       // function of three i32 values
}

impl Message            // we can define methods for enums
{
    fn call(&self)
    {
        // call body
    }
}

enum Option<T>          // enum Option with generic type, already in standard library
{
    Some(T),
    None,
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn match_value_in_cents(coin: Coin) -> u8
{
    match coin      // use the match expression with enums
    {               // the difference with an if statement is that if
                    // returns boolean, but match can return any type
        Coin::Penny =>  // we can also have code block in match arms
        {
            println!("Lucky Penny!!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main()
{
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    let new_home = ConciseIpAddrKind::V4(127, 0, 0, 1);
    let new_loopback = ConciseIpAddrKind::V6(String::from("::1"));

    route(ipv4);
    route(ipv6);

    let home = IpAddr
    {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr
    {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);              // using the Option enum with integers
    let some_string = Some("a string");     // and with strings

    let absent_number: Option<i32> = None;  // using the None type in the Option enum
                                            // also we have to provide the type to expect.
                                            // Compiler will not be able to guess the type
                                            // from a None value.
    
    
}
