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
    use std::thread;
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

    #[test(tokio::test)]
    async fn test_yield_example() {
        let a = async {
            let r1 = slow("a", 30); trpl::yield_now().await;
            let r2 = slow("a", 10); trpl::yield_now().await;
            let r3 = slow("a", 20); trpl::yield_now().await;
            vec![r1, r2, r3]
        };

        let b = async {
            let r1 = slow("b", 75); trpl::yield_now().await;
            let r2 = slow("b", 10); trpl::yield_now().await;
            let r3 = slow("b", 15); trpl::yield_now().await;
            let r4 = slow("b", 350); trpl::yield_now().await;
            vec![r1, r2, r3, r4]
        };

        let (result_a, result_b) = tokio::join!(a, b);
        assert_eq!(result_a, vec!["a:30", "a:10", "a:20"]);
        assert_eq!(result_b, vec!["b:75", "b:10", "b:15", "b:350"]);
    }

    fn slow(name: &str, ms: u64) -> String {
        thread::sleep(Duration::from_millis(ms));
        format!("{name}:{ms}")
    }

}

