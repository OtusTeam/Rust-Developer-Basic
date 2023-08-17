use test_config::Test;

#[tracer::trace]
fn foo(n: u32) -> String {
    bar(n + 1)
}

#[tracer::trace]
fn bar(a: u32) -> String {
    a.to_string()
}

#[derive(Debug, Test)]
struct Config {
    timeout_ms: u32,
    url: String,
}

fn main() {
    let s = foo(5);
    println!("s = {s}");

    let config = Config::test();
    println!("config = {config:?}");
}
