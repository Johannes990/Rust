fn main()
{
    let word = String::from("totaalselt lambitekst.");

    let word_from_function = first_word(&word);

    println!("first word is: {}", word_from_function);

    word.clear();   // should now give an error with slices

    println!("{}", word_from_function);
}

fn first_word(s: &str) -> &str  // since the type of s here is actually &str
                                // we can improve the first_word by
                                // changing (s: &String) to (s: &str) so we
                                // could directly pass string slices

{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}