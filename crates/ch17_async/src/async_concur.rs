use log::debug;
use std::time::Duration;

pub fn async_concur() {
    join_only();
    spawn_task();
    message_passing();
}

fn message_passing() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(10)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                debug!("Received: {}", val);
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}

// single task - wait with join
fn join_only() {
    debug!("========== join example ==========");

    trpl::block_on(async {
        let fut1 = async {
            for i in 1..5 {
                debug!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(100)).await
            }
        };

        let fut2 = async {
            for i in 1..2 {
                debug!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(100)).await
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

// spawn a new task and wait for it to complete (using await)
fn spawn_task() {
    debug!("========== spawn task example ==========");

    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..5 {
                debug!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(100)).await
            }
        });

        for i in 1..2 {
            debug!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(100)).await
        }

        handle.await.unwrap();
    });
}
