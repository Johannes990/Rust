use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();    // declare a new HashMap, data struct with key, value pairs.
                                        // like pythons dict
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // we can also make a new hashmap by using zip() and collect()
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // for key and value parameter types we use underscores, Rust can infer based on the data in vectors
    let scores_2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favourite color");
    let field_data = String::from("Green");

    let mut map = HashMap::new();

    map.insert(field_name, field_data); // now hashmap owns both strings, we can't call field_name or
                                        // field_data in our code. Strings can't be copied so they are moved into map
    println!("{}: {}", field_name, field_data);
}
