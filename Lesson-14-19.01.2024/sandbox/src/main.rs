struct Sandbox {
    errors: Vec<Box<dyn std::error::Error>>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    pub fn do_work<F, E: std::error::Error + 'static>(&mut self, f: F)
    where
        F: Fn() -> Result<(), E>
    {
        if let Err(err) = f() {
            self.errors.push(Box::new(err));
        }
    }

    pub fn drain_errors(&mut self) -> Vec<Box<dyn std::error::Error>> {
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

#[derive(Debug)]
enum Error {
    SomethingBadHappened,
    NotLucky,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn work_1() -> Result<(), std::io::Error> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, Error::SomethingBadHappened))
}

fn work_2() -> Result<(), Error> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    Err(Error::NotLucky)
}
