#[derive(Debug)]        // so we can inspect state
enum State
{
    Alabama,
    Alaska,
    // --snip--
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>
        {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>      // matching with Option<T>
{
    match x
    {
        //None => None,
        Some(i) => Some(i + 1),
    }
}

fn main()
{
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(State::Alaska);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{} is the value of penny in cents.", value_in_cents(penny));
    println!("{} is the value of nickel in cents.", value_in_cents(nickel));
    println!("{} is the value of dime in cents.", value_in_cents(dime));
    println!("{} is the value of quarter in cents.", value_in_cents(quarter));

    let some_u8_value = 0u8;
    match some_u8_value
    {
        1 => println!("one"),
        3 => println1("three"),
        5 => println1("five"),
        7 => println!("seven"),
        _ => ()     // use '_' as a placeholder for any value. We catch all values we care about
                    // befor the '_' case. Any value not matching, will be handled by the '_'.
                    // () is a unit value, so nothing happens. If we only care about one single case
                    // we should use if let instead.
    }
}
