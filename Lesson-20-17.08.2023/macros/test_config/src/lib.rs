pub use test_config_derive::Test;

pub trait Test {
    fn test() -> Self;
}

impl<T: Default> Test for T {
    fn test() -> Self {
        Self::default()
    }
}

