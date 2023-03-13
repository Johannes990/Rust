use std::io;

fn convert(x: f32, unit: String)
{
    if unit == "f"
    {
        println!("Converted temperature is {} Celsius.", (x - 32.0) * 0.5555555);
    }
    else if unit == "c"
    {
        println!("Converted temperature is {} Fahrenheit.", 32.0 + (x * 1.8));
    }
    else
    {
        println!("{unit} is not a recognized unit token!");
    }
}

fn main()
{
    let mut temp = String::new();
    let mut unit = String::new();

    println!("Please input temperature.");

    io::stdin().read_line(&mut temp).expect("Failed to read temperature!");
    let temp: f32 = match temp.trim().parse()
    {
        Ok(temp) => temp,
        Err(_) => 0.0,
    };

    println!("Please input unit as either 'f' or 'c'.");
    
    io::stdin().read_line(&mut unit).expect("Failed to read unit!");
    let unit: String = match unit.trim().parse()
    {
        Ok(unit) => unit,
        Err(_) => 'x'.to_string(),
    };

    println!("You entered {} as temperature and {} as unit.", temp, unit);

    convert(temp, unit);

}
