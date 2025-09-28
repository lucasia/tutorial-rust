use log::debug;
use std::thread;

pub fn unrecoverable() {
    debug!("starting unrecoverable!!!");

    let handle = thread::spawn(|| {
        debug!("worker thread starting");
        panic!("crash and burn!!");
    });

    match handle.join() {
        Ok(_) => debug!("worker thread finished cleanly"),
        Err(e) => {
            // Panic was contained to the thread
            debug!("worker thread panicked: {:?}", e);
        }
    }

    debug!("back on main thread — process still alive");
}
