#[cfg(test)]
mod tests {
    use test_log::test;

    type Kilometers = i32;

    #[test]
    fn test_types() {
        let x: i32 = 5;
        let y: Kilometers = 5;
        assert_eq!(x, y);
    }
}
