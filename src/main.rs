use davepoo_6502::{Mem, CPU};

fn main() {
        let mut mem: Mem = Mem::new();
        let mut cpu = CPU::new();
        cpu.reset(&mut mem);

        //given:
        mem[0xFFFC] = CPU::INS_LDA_IM;
        mem[0xFFFD] = 0x84;

        //when:
        cpu.execute(2, &mut mem);
}
