#[derive(Debug)]
struct TEST {
    pub version: u8,
    pub command: Command,
}

impl Test {
    fn makeTest(&self) {
        let test = Test {
            version: 1,
            command: Command::new(),
        };
    }
}
