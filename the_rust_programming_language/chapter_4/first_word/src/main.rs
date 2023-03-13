fn main()
{
    let mut s1 = String::from("Whatever you think.");

    let printable = first_word(&s1);    // printable will get the value 8

    println!("{}", printable);

    s1.clear();                         // this empties the String s1, making it equal to ""

    // printable still has value of 8 here, but there's no more string that
    // we could meaningfully ust the value 5 with, s1 is now totally invalid!
}

fn first_word(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }

    s.len()
}
