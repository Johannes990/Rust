use std::io;

fn n_fibonacci(n: u64) -> u64
{
    if n > 1
    {
        return n_fibonacci(n - 2) + n_fibonacci(n - 1);
    }
    else
    {
        return 1;
    }
}

fn binet(n: f64) -> f64
{
    let base: f64 = 5.0;
    let phi: f64 = (1.0 + base.sqrt()) / 2.0;
    let psi: f64 = (1.0 - base.sqrt()) / 2.0;
    return (phi.powf(n) - psi.powf(n)) / ( base.sqrt());
}

fn main()
{
    let mut n = String::new();

    println!("Please enter n for the n-th Fibonacci number to be returned...");

    io::stdin().read_line(&mut n).expect("Failed to read n!");
    let n: f64 = match n.trim().parse()
    {
        Ok(n) => n,
        Err(_) => 0.0,
    };

    println!("You entered number {} as n.", n);

    let fibonacci = binet(n);

    println!("The {}-th Fibonacci number is {}.", n, fibonacci);
}
