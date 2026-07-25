fn main() {
    let string = String::from("Hello World");
    let slice = first_word(&string);
    println!("{slice}");
}


// Uses &str fat pointer (Slice)
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    } 

    &s[..]
}

// Usable on bot &String and &str values
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    } 

    &s[..]
}