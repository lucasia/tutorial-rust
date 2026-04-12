use log::debug;
use std::thread;

pub fn threads() {
    move_example();
}

fn move_example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        debug!("Heres a vector: {v:?}");
    });

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;
    use log::{debug, info};

    const SLEEP_MS: u64 = 10;


    fn main_thread() {
        for i in 0..5 {
            debug!(">> hi number {i} from the main thread!");

            thread::sleep(Duration::from_millis(SLEEP_MS));
        }
    }

    fn spwan_counter(prefix: String, num_loops: i32) -> JoinHandle<Vec<String>> {
        let handle = thread::spawn(move || {
            let mut result: Vec<String> = Vec::new();

            for i in 0..num_loops {
                let msg = format!("{prefix} hi number {i} from the spawned thread!");
                debug!("{msg}");
                result.push(msg);
                thread::sleep(Duration::from_millis(SLEEP_MS));
            }

            result
        });

        handle
    }

    #[test_log::test]
    fn test_thread_name() -> Result<(), Box<dyn std::error::Error>> {
        let thread_name = "thread1";

        let handle = thread::Builder::new()
            .name(thread_name.to_string())
            .spawn(move || {
                thread::current().name().unwrap_or("").to_string()
            })?;

        let thread_result = handle
            .join()
            .map_err(|e| format!("thread panicked: {e:?}"))?;

        assert_eq!(thread_result, thread_name);

        Ok(())
    }

    #[test_log::test]
    fn test_multi_thread() -> Result<(), Box<dyn std::error::Error>> {
        let handle_thread1 = spwan_counter(String::from("####"), 5);
        let handle_thread2 = spwan_counter(String::from("!!!!"), 10);

        main_thread();

        let thread_result1 = handle_thread1.join().unwrap();
        info!("{:?}", thread_result1);
        assert_eq!(5, thread_result1.len());

        let thread_result2 = handle_thread2.join().unwrap();
        info!("{:?}", thread_result2);
        assert_eq!(10, thread_result2.len());

        Ok(())
    }
}
