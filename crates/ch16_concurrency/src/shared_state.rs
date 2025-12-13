use std::sync::{Arc, Mutex};
use std::thread;
use log::{debug, info};

pub fn shared_state() {
    single_thread_lock();
    multi_thread_lock();
}

fn multi_thread_lock() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
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

    info!("Result: {}", *counter.lock().unwrap());
}

fn single_thread_lock() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    debug!("m = {:?}", m);
}