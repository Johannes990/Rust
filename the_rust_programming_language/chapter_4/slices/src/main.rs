fn main()
{
    let s = String::from("Hello, world");
    let s2 = String::from("random string");
    let s3 = String::from("New random string");

    let hello = &s[0..5];   // slicing in Rust
    let world = &s[6..12];  // very similar to python];

    println!("{} ... {}", hello, world);

    let range1 = &s2[0..4];
    let range2 = &s2[..4];  // we can skip the zero in [0..4] to give [..4]

    println!("{} ... {}", range1, range2);

    let len = s3.len();

    let ran1 = &s3[0..len];
    let ran2 = &s3[0..];    // if slice includes the last byte of the String
                            // you can drop the trailing number
    let ran3 = &s3[..];     // dropping both values returns the entire String

    println!("{} ... {} and {}", ran1, ran2, ran3);
}
