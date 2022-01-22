use davepoo_6502::{Mem, CPU};

fn main() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);
    mem[0xFFFC] = CPU::INS_JSR;
    mem[0xFFFD] = 0x42;
    mem[0xFFFE] = 0x42;
    mem[0x4242] = CPU::INS_LDA_IM;
    mem[0x4243] = 0x84;
    //end - inline a little program
    cpu.execute(8, &mut mem);
    mem[0x0042] = 0x84;
}
