use std::collections::HashMap;
use std::io;

fn create_vector() -> Vec<i32> {
    println!("Please enter the number of integers you are adding to vector.");
    let mut return_vector = Vec::new();
    let mut num_count = String::new();

    io::stdin().read_line(&mut num_count).expect("Failed to read input!");
    let num_count: i32 = match num_count.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    for idx in 0..num_count {
        println!("Please enter the {}-th number:", idx);
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read input!");
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        return_vector.push(num);
    }
    return_vector
}

fn mean(numbers: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    let size: f64 = numbers.len() as f64;

    for element in numbers {
        let e_f64: f64 = *element as f64;
        sum += e_f64;
    }
    sum / size
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut sorted_numbers: Vec<i32> = numbers.to_vec();
    sorted_numbers.sort();
    let index: usize = sorted_numbers.len() / 2;
    let return_element = sorted_numbers.get(index);

    match return_element {
        None => 0,
        Some(i) => *i,
    }
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut vector_map: HashMap<i32, i32> = HashMap::new();
    let mut max_count = 0;
    let mut return_value = 0;

    for element in numbers {
        let count = vector_map.entry(*element).or_insert(0);
        *count += 1;
    };

    for (key, value) in vector_map.iter() {
        if value >= &max_count {
            max_count = *value;
            return_value = *key;
        }
    };
    return_value
}

fn main() {
    let vector = create_vector();

    println!("{:?}", vector);
    println!("The mean of the elements of this vector is {}", mean(&vector));
    println!("The median of the elements of this vector is: {}", median(&vector));
    println!("The mode of the elements of this vector is: {}", mode(&vector));
}
