#[macro_export]
macro_rules! info {
    ($b:ident, $l:literal) => {
        $b.push(format!("INFO: {}", $l))
    };
    ($b:ident, $l:literal, $($e:expr),* $(,)?) => {
        $b.push(format!("INFO: {}", format!($l, $($e),*)))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn info_works() {
        let mut buffer = vec![];
        info!(buffer, "Hello");
        assert_eq!(buffer.first(), Some(&String::from("INFO: Hello")));
    }

    #[test]
    fn info_with_format_works() {
        let mut buffer = vec![];
        let a = 5;
        info!(buffer, "Hello {}", a);
        assert_eq!(buffer.first(), Some(&String::from("INFO: Hello 5")));
    }

    #[test]
    fn info_with_format_and_literal_works() {
        let mut buffer = vec![];
        let a = 5;
        info!(buffer, "Hello {} {}", a, "World",);
        assert_eq!(buffer.first(), Some(&String::from("INFO: Hello 5 World")));
    }

    #[test]
    fn hygiene() {
        let mut a = 5;
        assert_eq!(a, 5);

        macro_rules! my_macro {
            () => {
                a = 10;
            };
        }

        my_macro!();
        assert_eq!(a, 10);
    }
}
