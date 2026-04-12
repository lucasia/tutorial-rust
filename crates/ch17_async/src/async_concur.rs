use log::debug;
use std::time::Duration;

pub fn async_concur() {
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


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use log::debug;
    use test_log::test;

    // spawn a new task and wait for it to complete (using await)
    #[test(tokio::test)]
    async fn test_spawn_task() {
        let task_id = 1u32;
        let handle = trpl::spawn_task(async move {
            let mut result: Vec<String> = Vec::new();

            for i in 0..5 {
                let msg = format!("task_{task_id}: hi number {i} from async task!");
                debug!("{msg}");
                result.push(msg);
                trpl::sleep(Duration::from_millis(10)).await
            }
            result
        });

        for i in 0..2 {
            debug!("main: hi number {i} !");
            trpl::sleep(Duration::from_millis(10)).await
        }

        let result = handle.await.unwrap();

        assert_eq!(5, result.len());
    }

    #[test(tokio::test)]
    async fn test_join() {
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
    }

}