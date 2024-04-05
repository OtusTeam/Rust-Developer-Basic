fn foo(v: &[u32]) -> Option<u32> {
    let last = v.last()?;
    Some(last * 2)
}

fn main() {
    let v = vec![1];
    let value = foo(&v);
    dbg!(value);

    let v2 = vec![1, 2, 3];
    let value2 = foo(&v2);
    dbg!(value2);

    // let value3 = match (value, value2) {
    //     (None, None) => None,
    //     (None, Some(x)) | (Some(x), None) | (Some(x), Some(_)) => Some(x + 1),
    // };

    let value3 = value.or(value2).map(|val| val + 1).unwrap();
    dbg!(value3);

    let b = bar().unwrap();
    dbg!(b);
}

fn bar() -> Result<u64, String> {
    let a: u64 = baz().map(|val| val.into()).map_err(|err| err.to_string())?;
    Ok(a)
}

fn baz() -> Result<u32, u32> {
    Ok(5)
}
