fn main() -> Result<(), eyre::Error> {
    let world = "World";
    let s = format!("Hello `{world}`");
    dbg!(s);

    match main_function() {
        x @ Ok(()) => x,
        Err(Error::Baz(x)) => {
            println!("Baz failed with {x}");
            Ok(())
        }
        Err(err) => Err(err.into()),
    }?;

    Ok(())
}

fn main_function() -> Result<(), Error> {
    // foo().map_err(|s| Error::Foo(s))?;
    // bar()?;
    too()?;
    baz()
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("foo failed")]
    Foo(String),
    #[error("bar failed")]
    Bar,
    #[error("baz failed")]
    Baz(u32),
    #[error("too failed")]
    Too(#[from] TooError)
}

fn foo() -> Result<(), String> {
    Err(String::from("foo failed"))
}

fn bar() -> Result<(), Error> {
    Err(Error::Bar)
}

fn baz() -> Result<(), Error> {
    Err(Error::Baz(5))
}

#[derive(Debug, thiserror::Error)]
enum TooError {
    #[error("Some1")]
    Some1,
}

fn too() -> Result<(), TooError> {
    Err(TooError::Some1)
}
