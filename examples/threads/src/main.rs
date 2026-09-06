use std::thread;
use std::time::Duration;

fn main() {

    // Will wait for the computation to finish
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi handler number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi handler number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();    

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }


    // We need to move owernship of values to use them in a thread
    // Otherwise a thread may use a freed value
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

    // Demonstrate that the thread captures but does not change the value
    let mut n = 1;

    let t = thread::spawn(move || {

        n = n + 1;

        thread::spawn(move || {
            n = n + 1;

        })

    });

    n = n + 1;

    t.join().unwrap().join().unwrap();

    println!("{n}");
}
