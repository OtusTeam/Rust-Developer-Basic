pub use test_value_derive::TestValue;

pub trait TestValue {
    fn test_value() -> Self;
}
