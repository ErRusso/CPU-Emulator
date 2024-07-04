struct CPU{
  a: i16,
  b: i16,

  ip: i16

}

impl CPU{

  fn init(&mut self){
    self.a = 0; self.b = 0; self.ip = 0;
  }
  
  fn print_status(&mut self){
    println!("a: {}, b: {}, ip: {}", self.a, self.b, self.ip);
  }

  fn mov_into_register(&mut self, memory:&mut MEMORY){
    match memory.get(self.ip + 1) {
      0x0 => self.a = memory.get(self.ip + 2),
      0x1 => self.b = memory.get(self.ip + 2),
      _ => panic!("Invalid Register")
    }
    self.ip = self.ip + 3;
  }

  fn mov_next_instruction(&mut self){
    self.ip = self.ip + 1;
  }
  
  fn fetch_instruction(&mut self, memory:&mut MEMORY) -> i16{
    memory.get(self.ip)
  }

  fn run(&mut self, memory:&mut MEMORY){
    match self.fetch_instruction(memory) {
      0x0 => self.mov_next_instruction(),
      0x1 => self.mov_into_register(memory),
      0x2 => self.add(),
      _ => panic!("Invalid Instruction")
    }
  }
  
  fn add(&mut self){
    self.a = self.a + self.b;
    self.ip = self.ip + 1;
  }
}