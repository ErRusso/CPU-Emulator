include!("../include/cpu.rs");
include!("../include/memory.rs");
fn main() {
    println!("Starting Emulation");
    let mut cpu:CPU = CPU{a:0,b:0,ip:0};
    let mut memory:MEMORY = MEMORY { memory: [0; 256].to_vec() };
    memory.init();
    cpu.init();

    memory.load(0x0, 0x1); // mov
    memory.load(0x1, 0x0); // a
    memory.load(0x2, 0x3); // 0x3
    memory.load(0x3, 0x1); // mov
    memory.load(0x4, 0x1); // b
    memory.load(0x5, 0x6); // 0x6
    memory.load(0x6, 0x2); // add
    memory.load(0x7, 0x3); // put
    memory.load(0x8, 0xa); // 0xF

    memory.print_from_to(0, 0xb);

    for i in 0../*128*/4{
        cpu.print_status();
        cpu.run(&mut memory);
    }

    memory.print_from_to(0, 0xb);
}
