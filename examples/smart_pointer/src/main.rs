enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    
    // Box<T> smart pointer (implements Deref and Drop traits)
    let b = Box::new(5);
    println!("box = {b}");

    // Box permits recursion (recursion uses Cons from Lisp)
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    // Deref
    let m = Box::new(String::from("Rust"));
    hello(&m);

}


fn hello(name: &str) {
    println!("Hello, {name}!");
}
