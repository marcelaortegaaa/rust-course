// Task: implement a mutex protected counter that atomically increments

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let _value = {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
                *counter
            };
            println!("Increased counter by 1");
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = Arc::try_unwrap(counter).unwrap().into_inner().unwrap();
    println!("Final counter: {final_value}");
}
