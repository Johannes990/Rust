use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);
    scores.insert(String::from("Violet"), 40);

    let team_name = String::from("Violet");
    let score = scores.get(&team_name);

    println!("team: {}, score: {:?}", team_name, score);

    for (key, value) in &scores {
        println!("team: {}, score: {}", key, value);
    }

    // updating a hashmap - overwriting old values
    scores.insert(String::from("Green"), 5);    // key 'Green' with value 5 in hashmap
    scores.insert(String::from("Green"), 35);   // now key 'Green' has value 35, overwriting the old value 5

    // updating a hashmap, only inserting a value, if key has no value beforehand
    // use entry().or_insert() - entry checks wether a specific key has a value
    // associated with it, and returns it, if no value is found, a value is inserted
    // using the or_insert() function
    let mut new_scores = HashMap::new();

    new_scores.insert(String::from("Blue"), 10);

    new_scores.entry(String::from("Yellow")).or_insert(25);
    new_scores.entry(String::from("Blue")).or_insert(15);

    println!("{:?}", new_scores);

    // updating a hashmap - updating to a new value based on the old value
    let text = "Hello, my beautiful world, what a nice beautiful day. Where in the world are we? Oh my my my.";

    let mut word_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);  // entry returns a mutable reference
        *count += 1;                                    // so to update count, we have to dereference using *
    }

    println!("{:?}", word_map)
}
