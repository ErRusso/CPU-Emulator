struct CPU {
    a: i16,
    b: i16,
    c: i16,
    ip: i16,
}

impl CPU {
    fn init(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
    }

    fn print_status(&mut self) {
        println!(
            "cpu: a: {}, b: {}, c: {}, ip: {}",
            self.a, self.b, self.c, self.ip
        );
    }

    fn mov_into_register(&mut self, memory: &mut MEMORY) {
        match memory.get(self.ip + 1) {
            0x0 => self.a = memory.get(self.ip + 2),
            0x1 => self.b = memory.get(self.ip + 2),
            _ => panic!("Invalid Register"),
        }
        self.ip = self.ip + 3;
    }

    fn add(&mut self) {
        self.a = self.a + self.b;
        self.ip = self.ip + 1;
    }

    fn sub(&mut self) {
        self.a = self.a - self.b;
        self.ip = self.ip + 1;
    }

    fn mol(&mut self) {
        self.a = self.a * self.b;
        self.ip = self.ip + 1;
    }

    fn div(&mut self) {
        self.a = self.a / self.b;
        self.c = self.a % self.b;
        self.ip = self.ip + 1;
    }

    fn mov_into_memory(&mut self, memory: &mut MEMORY) {
        let address = memory.get(self.ip + 1);
        memory.load(address, self.a);
        self.ip = self.ip + 2;
    }

    fn mov_next_instruction(&mut self) {
        self.ip = self.ip + 1;
    }

    fn jmp(&mut self, memory: &mut MEMORY) {
        self.ip = memory.get(self.ip + 1);
    }

    fn fetch_instruction(&mut self, memory: &mut MEMORY) -> i16 {
        memory.get(self.ip)
    }

    fn run(&mut self, memory: &mut MEMORY) {
        match self.fetch_instruction(memory) {
            0x0 => self.mov_next_instruction(),    // nop
            0x1 => self.mov_into_register(memory), // mov
            0x2 => self.add(),
            0x3 => self.sub(),
            0x4 => self.mol(),
            0x5 => self.div(),
            0x6 => self.mov_into_memory(memory), // put
            0x7 => self.jmp(memory),
            _ => panic!("Invalid Instruction"),
        }
    }
}
