fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly
    let s2 = "initial contents".to_string();

    println!("{}", s);
    println!("{}", s2);

    let mut s1 = String::from("foo");
    let s3 = "bar";
    s1.push_str(s3); // now s3 = "foobar", use push_str to add string slices
    println!("{}", s1);

    let mut stringy = String::from("lo");
    stringy.push('l'); // now stringy = "lol", use string.push('') to add single characters
    println!("{}", stringy);

    let text1 = String::from("Hello, ");
    let text2 = String::from("World!");
    let text3 = text1 + &text2; // note text1 has been moved here and can no longer be used
    println!("{}", text3);

    let t1 = String::from("Tic");
    let t2 = String::from("Tac");
    let t3 = String::from("Toe");

    let s = t1 + "-" + &t2 + "-" + &t3; // + operator get unwieldy
    println!("The string in s is now: {}", s);

    let t11 = String::from("Tic");

    let s = format!("Now three strings seprarately formated: {}-{}-{}", t11, t2, t3); // better to use format maybe...
    println!("{}", s);
}
