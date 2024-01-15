use std::collections::HashMap;
use std::io;


fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    println!{"Please enter a command:"}
    println!("either in the form of: Add (name) to (department)");
    println!("or in the form: Get (department)");
    println!("Enter 'quit' to exit program\n");

    loop {
        println!("input:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let command_tokens: Vec<&str> = input.split_whitespace().collect();
        let query_department = command_tokens[command_tokens.len() - 1];
        let query_command = command_tokens[0].to_lowercase();

        match query_command.as_str() {
            "add" => {
                let person = String::from(command_tokens[1]);
                if departments.contains_key(query_department) {
                    let person_list = departments.get_mut(query_department).unwrap();
                    person_list.push(person);
                } else {
                    let mut new_person_list = Vec::new();
                    new_person_list.push(person);
                    departments.insert(query_department.to_string(), new_person_list);
                }
            },
            "get" => {
                if departments.contains_key(query_department) {
                    let person_list = departments.get(query_department).unwrap();
                    println!("{:?}", person_list);
                } else {
                    println!("No such department\n")
                }
            },
            "quit" => {
                break;
            }
            _ => {
                println!("Error reading command. Please enter a new command.")
            },
        }

    }
}
