use log::debug;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

const SLEEP_MS: u64 = 100;

pub fn threads() {
    multi_thread();
    move_example();
}

fn move_example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        debug!("Heres a vector: {v:?}");
    });

    handle.join().unwrap();
}

fn multi_thread() {
    let handle_thread1 = spawn(String::from("####"), 5);
    let handle_thread2 = spawn(String::from("!!!!"), 10);

    main_thread();

    handle_thread1.join().unwrap();
    handle_thread2.join().unwrap();
}

fn main_thread() {
    for i in 0..5 {
        debug!(">> hi number {i} from the main thread!");

        thread::sleep(Duration::from_millis(SLEEP_MS));
    }
}

fn spawn(prefix: String, num_loops: i32) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        for i in 1..num_loops {
            debug!("{prefix} hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(SLEEP_MS));
        }
    });

    handle
}
