// TODO - streams https://doc.rust-lang.org/book/ch17-04-streams.html


#[cfg(test)]
mod tests {
    use log::info;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_streams() {

        info!("hello world from test")

    }
}