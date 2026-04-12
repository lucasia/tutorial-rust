// TODO - streams https://doc.rust-lang.org/book/ch17-04-streams.html

#[cfg(test)]
mod tests {
    use log::info;
    use test_log::test;
    use trpl::StreamExt;

    #[test(tokio::test)]
    async fn test_stream_as_iterator() {
        let values = vec![1, 2, 3, 4, 5];

        let iter = values.iter().map(|x| x * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            info!("The value of value was: {value}");
        }
    }
}
