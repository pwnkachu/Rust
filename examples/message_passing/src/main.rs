use std::sync::mpsc;
use std::thread;

fn main() {
    // Channels can only send values of a single type
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("message");
        tx.send(val).unwrap();
    });

    // recv will wait until the message arrives
    // For asynchronous message passing use try_recv
    let received = rx.recv().unwrap();
    println!("Received : {received}")


    /*
    This code does not work because we moved the ownership before
    the println.

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {val}");
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
     */
}