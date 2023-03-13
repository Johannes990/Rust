#[derive(Debug)]
enum SpreadsheetCell {      // all entries under enum are considered the same enum type
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![         // vector of data of type enum SpreadsheetCell
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(20.343),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
