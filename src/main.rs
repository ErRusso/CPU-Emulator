include!("./include/cpu.rs");
include!("./include/memory.rs");
fn main() {
    println!("Starting Emulation");
    let mut cpu: CPU = CPU {
        a: 0,
        b: 0,
        c: 0,
        ip: 0,
    };
    let mut memory: MEMORY = MEMORY {
        memory: [0; 256].to_vec(),
    };
    memory.init();
    cpu.init();

    // (TEST MATH)
    // mov a, 10
    memory.load(0x0, 0x1);
    memory.load(0x1, 0x0);
    memory.load(0x2, 0xa);
    // mov b, 22
    memory.load(0x3, 0x1);
    memory.load(0x4, 0x1);
    memory.load(0x5, 0x16);
    // add
    memory.load(0x6, 0x2);
    // put a, 128
    memory.load(0x7, 0x6);
    memory.load(0x8, 0x80);
    // sub
    memory.load(0x9, 0x3);
    // put a, 129
    memory.load(0xa, 0x6);
    memory.load(0xb, 0x81);
    // mol
    memory.load(0xc, 0x4);
    // put a, 130
    memory.load(0xd, 0x6);
    memory.load(0xe, 0x82);
    // mov a, 22
    memory.load(0xf, 0x1);
    memory.load(0x10, 0x0);
    memory.load(0x11, 0x16);
    // div
    memory.load(0x12, 0x5);
    // put a, 131
    memory.load(0x13, 0x6);
    memory.load(0x14, 0x83);

    // HLT
    memory.load(0x15, 0xFF);

    for _ in 0..128 {
        cpu.print_status();
        if cpu.fetch_instruction(&mut memory) == 0xFF {
            println!("HLT");
            break;
        }
        cpu.run(&mut memory);
    }
    memory.print_from_to(128, 133);
}
