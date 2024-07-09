struct MEMORY {
    memory: Vec<i16>,
}

impl MEMORY {
    fn init(&mut self) {
        self.memory = vec![0; 256]; //128 istruction - 128 data
    }

    fn load(&mut self, address: i16, value: i16) {
        self.memory[address as usize] = value;
    }

    fn get(&mut self, address: i16) -> i16 {
        self.memory[address as usize]
    }

    fn print_from_to(&mut self, start: i16, end: i16) {
        print!("Memory: ");
        for i in start..end {
            print!("{},", self.memory[i as usize]);
        }
        println!("");
    }
}
