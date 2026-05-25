#[cfg(test)]
mod tests {
    use crate::my_vec;
    use ch20_hello_macro::HelloMacro;
    use ch20_hello_macro_derive::HelloMacro;
    use log::debug;
    use test_log::test;

    // ================================================
    // macro_rules! macros
    // ================================================

    #[macro_export]
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    #[test]
    fn test_declarative_macros() {
        let v: Vec<u32> = my_vec![1, 2, 3];

        assert_eq!(v, vec![1, 2, 3]);
    }

    // ================================================
    // derive macros
    // ================================================

    #[derive(HelloMacro)]
    struct Pancakes;

    #[test]
    fn test_hello_macro() {
        let result = Pancakes::hello_macro();
        debug!("result: {}", result);

        assert_eq!("Hello, Macro!  My name is Pancakes!", result);
    }
}
