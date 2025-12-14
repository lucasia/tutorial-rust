use std::time::Duration;
use log::debug;

pub fn async_concur() {
    join_only();
    spawn_task();
}

// single task - wait with join
fn join_only() {
    debug!("========== join example ==========");

    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                debug!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await
            }
        };

        let fut2 = async {
            for i in 1..5 {
                debug!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await
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
            for i in 1..10 {
                debug!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await
            }
        });

        for i in 1..5 {
            debug!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await
        }

        handle.await.unwrap();
    });
}
