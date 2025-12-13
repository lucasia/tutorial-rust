use std::sync::mpsc;
use std::thread;
use std::time::Duration;
/*
 * Go language slogan: “Do not communicate by sharing memory; instead, share memory by communicating.”
 * channels : transmitter -> receiver
 */
pub fn messages() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
