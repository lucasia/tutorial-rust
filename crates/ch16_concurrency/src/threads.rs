use log::debug;
use std::thread;

pub fn threads() {
    move_example();
}

fn move_example() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        debug!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::thread;
    use log::debug;
    use test_log::test;
    use anyhow::{anyhow, Result};

    fn generate_greetings(num_loops: i32) -> Vec<String> {
        let thread_name = thread::current().name().unwrap_or("").to_string();
        let mut result: Vec<String> = Vec::new();

        for i in 0..num_loops {
            let msg = format!("{thread_name}: hello number {i}!");
            result.push(msg);
        }

        result
    }

    #[test]
    fn test_spawn() {
        let msg = "hello world!";
        let main_thread_id = thread::current().id();

        let handle = thread::spawn(move || {
            assert_ne!(thread::current().id(), main_thread_id);
            // we can do whatever we want here
            msg
        });

        assert_eq!(handle.join().unwrap(), msg);
    }

    #[test]
    fn test_multi_thread() -> Result<()> {
        let (mut result1, mut result2) = (vec![], vec![]);

        thread::scope(|s| -> Result<()> {
            let h1 = thread::Builder::new()
                .name("foo".to_string())
                .spawn_scoped(s, || generate_greetings(5))?;
            let h2 = thread::Builder::new()
                .name("bar".to_string())
                .spawn_scoped(s, || generate_greetings(10))?;

            result1 = h1.join().map_err(|e| anyhow!("thread1 panicked: {e:?}"))?;
            result2 = h2.join().map_err(|e| anyhow!("thread2 panicked: {e:?}"))?;

            Ok(())
        })?;

        debug!("{result1:?} {result2:?} ");

        assert_eq!(5, result1.len());
        assert_eq!(10, result2.len());


        Ok(())
    }
}
