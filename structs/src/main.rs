fn main() {
    let mut user = build_user("mail@mail.com".to_string(), "username".to_string());
    println!("{0}", user.email);

    update_email("mail2@mail.com".to_string(), &mut user);

    println!("{0}", user.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Uses lifetimes, that allows a struct store references instead of owned data.
struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_email(email: String, user: &mut User){
    user.email = email;
}

/*   TUPLE STRUCTS   */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/*    UNIT-LIKE STRUCTS   */
struct AlwaysEqual;