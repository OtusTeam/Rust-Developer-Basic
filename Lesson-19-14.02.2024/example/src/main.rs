use tracer::trace;
use test_value::TestValue;

#[trace]
fn foo(n: u32) -> String {
    bar(n + 1)
}

#[trace]
fn bar(n: u32) -> String {
    n.to_string()
}

#[derive(Debug)]
struct SubsystemConfig {
    timeout_ms: u32,
    url: String,
}

impl TestValue for SubsystemConfig {
    fn test_value() -> Self {
        Self {
            timeout_ms: 1000,
            url: String::from("example.com"),
        }
    }
}

#[derive(Debug, TestValue)]
struct Config {
    subsystem: SubsystemConfig,
    subsystem_2: SubsystemConfig,
}

fn main() {
    let s = foo(5);
    println!("s = {s}");

    let config = Config::test_value();
    println!("config = {config:#?}");
}
