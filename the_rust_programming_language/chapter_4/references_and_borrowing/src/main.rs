fn main()
{
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);    // give reference of s1 as argument
                                        // instead of the actual variable s1
                                        // so s1 remains in scope

    println!("The length of {} is {}", s1, len);

    let s2 = String::from("Hello");

    // change(&s2);    // change() tries to change data it has no ownership of. Error!

    let mut s3 = String::from("Hello"); // s3 has to be mut for reference borrowing to work

    change_2(&mut s3);  // change_2() takes a mutable reference now. You can only
                    // have one mutable reference to a prticular piece of data
                    // in a particular scope.
    println!("{}", s3);

    let r1 = &mut s3;
    let r2 = &mut s3;           // already borrowed s3 as mutable on previous line
    println!("{}, {}", r1, r2); // so r2 cannot be used since s3 would be borrowed
                                // as mutable for the second time

    let mut new_string = String::from("Hello");

    let a1 = &new_string;   // no problem
    let a2 = &new_string;   // no problem
    let a3 = &mut new_string;   // BIG PROBLEM

    println!("{}, {}, and {}", a1, a2, a3); // cannot combine mutable and immutable
                                            // references for same variable
}

fn calculate_length(s: &String) -> usize    // s is a reference to a String
{
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership
    // of what it refers to, nothing happens.

// fn change(some_string: &String)      // this should give us an error
// {
//     some_string.push_str(", world");
// }

fn change_2(some_string: &mut String)
{
    some_string.push_str(", world");
}