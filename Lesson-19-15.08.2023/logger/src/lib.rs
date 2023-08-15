#[macro_export]
macro_rules! info {
    ($buffer:ident, $fmt:expr) => {
        $buffer.push(format!("INFO: {}", ::std::format_args!($fmt)))
    };
    ($buffer:ident, $fmt:expr, $($args:tt)*) => {
        $buffer.push(format!("INFO: {}", ::std::format_args!($fmt, $($args)*)))
    };
}

#[macro_export]
macro_rules! warn {
    ($buffer:ident, $fmt:expr) => {
        $buffer.push(format!("WARNING: {}", ::std::format_args!($fmt)))
    };
    ($buffer:ident, $fmt:expr, $($args:tt)*) => {
        $buffer.push(format!("WARNING: {}", ::std::format_args!($fmt, $($args)*)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_basic_works() {
        let mut v: Vec<String> = vec![];
        info!(v, "Hello");
        assert_eq!(v.first().unwrap(), &String::from("INFO: Hello"));
    }

    #[test]
    fn info_format_works() {
        let mut v: Vec<String> = vec![];
        let a = 5;
        info!(v, "Hello, a = {}", a);
        assert_eq!(v.first().unwrap(), &String::from("INFO: Hello, a = 5"));
    }

    #[test]
    fn warn_basic_works() {
        let mut v: Vec<String> = vec![];
        warn!(v, "Hello");
        assert_eq!(v.first().unwrap(), &String::from("WARNING: Hello"));
    }

    #[test]
    fn warn_format_works() {
        let mut v: Vec<String> = vec![];
        let a = 5;
        warn!(v, "Hello, a = {}", a);
        assert_eq!(v.first().unwrap(), &String::from("WARNING: Hello, a = 5"));
    }

    #[test]
    fn hygiene_check() {
        let a = 25;

        macro_rules! string_from_a {
            () => {
                format!("{}", a)
            };
        }

        let s = string_from_a!();
        assert_eq!(s, "25");
    }
}
