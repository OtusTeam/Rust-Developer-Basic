enum FooError {
    Bar(BarError),
    Baz(BazError),
}

impl From<BarError> for FooError {
    fn from(error: BarError) -> Self {
        FooError::Bar(error)
    }
}

impl From<BazError> for FooError {
    fn from(error: BazError) -> Self {
        FooError::Baz(error)
    }
}

impl TryFrom<FooError> for BarError {
    type Error = &'static str;

    fn try_from(value: FooError) -> Result<Self, Self::Error> {
        if let FooError::Bar(error) = value {
            Ok(error)
        } else {
            Err("Not a BarError")
        }
    }
}

struct BarError;

struct BazError;

fn foo() -> Result<(), FooError> {
    bar()?;
    baz()?;
    Err(BarError.into())
}

fn bar() -> Result<(), BarError> {
    Ok(())
}

fn baz() -> Result<(), BazError> {
    Ok(())
}

#[test]
fn test() {
    let err = foo().unwrap_err();
    let _: Result<BarError, _> = err.try_into();

    let a: u32 = 5;
    let b: usize = a.try_into().expect("u32 is always convertible to usize");
}
