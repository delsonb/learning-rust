use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    multiple_ownership_with_multiple_threads();
}

fn multiple_ownership_with_multiple_threads() {
    let counter = Arc::new(Mutex::new(0)); // Rc<T> wouldn't work because it isn't atomic
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// fn sharing_a_value_using_a_mutex() {
//     let counter = Mutex::new(0);
//     let mut handles - vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

fn mutex_basics() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();  // lock() returns a MutexGuard, which is a smart pointer
        *num = 6;
    }

    println!("m = {:?}", m);
}
