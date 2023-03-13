struct User
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main()
{
    let user1 = User
    {
        email: String::from("someone@example.com"),
        username: String::from("John Doe"),
        active: true,
        sign_in_count: 1
    };

    let mut user2 = User
    {
        username: String::from("Mary Jane"),
        email: String::from("mary.jane@google.com"),
        active: true,
        sign_in_count: 1,
    };

    user2.username = String::from("Mees Metsast");

    let user1_1 = User      // create a new user using some of the data from user1
    {
        username: String::from("Anton Kabi"),
        email: String::from("lambimail@234"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user1_2 = User      // create new user using the struct update syntax
    {
        username: String::from("Anton Kabi"),
        email: String::from("lambimail@234"),
        ..user1             // .. specifies that the fields not explicitly set should
                            // have the same value as in the given instance, user1
    };

    println!("User1_1:");
    print_user(&user1_1);
    println!("\nUser1_2:");
    print_user(&user1_2);

    let user3 = build_user(String::from("Maarika Valss"), String::from("maarika@hot.ee"));
    let user4 = build_user(String::from("Teet Piho"), String::from("Härgatäis@google.com"));
    let user5 = build_user(String::from("Mints Pints"), String::from("mints.pints@mail.ee"));
    let user6 = build_user_2(String::from("Johannes Jürgenson"), String::from("johannes.jyrgenson@gmail.com"));

    let users = [user1, user2, user3, user4, user5, user6];

    for (index, user) in users.iter().enumerate()
    {
        println!("User {} is:", index);
        print_user(&user);
    }
}

fn build_user(name: String, email: String) -> User
{
    User
    {
        username: name,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

fn build_user_2(username: String, email: String) -> User    // build_user init shorthand
{
    User
    {
        username,               // we can write username and email only once, because they
        email,                  // have the same name as the struct fields.
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User)
{
    println!("Users name: {}", user.username);
    println!("Users email address: {}", user.email);
    println!("Log in count: {}", user.sign_in_count);
    println!("Is this user active: {}", user.active);
}