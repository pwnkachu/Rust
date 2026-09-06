fn main() {
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// Specifies that all the references must have the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// If a struct holds a reference it must specify a lifetime
// Basically the struct can't outlive the lifetime of part
struct ImportantExcerpt<'a> {
    part: &'a str,
}