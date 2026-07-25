#[cfg(test)]
mod tests {
    #[test]
    fn test_reborrow_len() {
    let mut vec = vec![1, 2, 3];
    let v: &mut Vec<i32> = &mut vec; // [R, W]

    // EXPLICIT REBORROW:
    // `&*v` deference 'v' (*v) and takes an immutable reference (&)
    // Durin len() the reference has only the R permit
    let len = Vec::len(&*v); 

    // works
    v.push(10); 

    let mut vec = vec![1, 2, 3];
    let v = &mut vec;

    v.push(4); // implicit reborrow
    let l = v.len(); // Equals to: let l = Vec::len(&*v);

    }
}