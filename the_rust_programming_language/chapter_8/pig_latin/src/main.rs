use std::io;

fn main() {
    let _consonants = "qwrtypsdfghjklzxcvbnm";
    let _vowels = "aeiou";
    let mut input = String::new();
    let mut word_vector = Vec::new();
    let mut output = String::new();

    println!("Please enter text to convert into pig latin.");

    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");

    println!("Entered text:\t{}", input);

    for word in input.split_whitespace() {
        let first_letter = word[0..1].to_lowercase();
        if _consonants.contains(&first_letter) {
            let mut new_string = String::from(&word[1..word.len()]);
            new_string.push('-');
            new_string.push_str(&first_letter);
            new_string.push_str("ay");
            word_vector.push(new_string);
        }
        if _vowels.contains(&first_letter) {
            let mut new_string = String::from(word);
            new_string.push_str("-hay");
            word_vector.push(new_string);
        }
    }

    for idx in 0..word_vector.len() {
        output.push_str(&word_vector[idx]);
        if idx != word_vector.len() - 1 {
            output.push(' ');
        }
    }

    println!("{}.", output);
}
