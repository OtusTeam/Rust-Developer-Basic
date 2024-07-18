struct Sandbox {
    errors: Vec<Box<dyn std::fmt::Debug>>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn do_work<E>(&mut self, f: impl Fn() -> Result<(), E>)
    where
        E: std::fmt::Debug + 'static,
    {
        if let Err(err) = f() {
            self.errors.push(Box::new(err));
        }
    }

    pub fn drain_errors(&mut self) -> Vec<Box<dyn std::fmt::Debug>> {
        std::mem::take(&mut self.errors)
    }
}

fn main() {
    let mut sandbox = Sandbox::new();

    for _ in 0..2 {
        sandbox.do_work(work_1);
    }

    for _ in 0..2 {
        sandbox.do_work(work_2);
    }

    let errors = sandbox.drain_errors();
    dbg!(errors);
}

fn work_1() -> Result<(), String> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    Err("Error 1".to_owned())
}

#[derive(Debug)]
enum Error {
    SomethingWrong,
    BadLuck,
}

fn work_2() -> Result<(), Error> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    Err(Error::BadLuck)
}
