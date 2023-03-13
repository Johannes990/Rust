fn main() {
    for c in "नमस्ते".chars() {
        println!("{}", c);  // returns single unicode characters
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);  // returns raw byte values, here the unicode values for a single char are
                            // made up of more than a single byte
    }
}