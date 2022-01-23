use davepoo_6502::{Mem, CPU, s32};

fn main() {
    /*
        //set up;
        let mut mem: Mem = Mem::new();
        let mut cpu = CPU::new();
        cpu.reset(&mut mem);

        //given:
        cpu.set_x(0xFF);
        mem[0xFFFC] = CPU::INS_LDA_ABSX;
        mem[0xFFFD] = 0x02;
        mem[0xFFFE] = 0x44; //0x4402
        mem[0x4501] = 0x37; //0x4402+0xFF crosses page boundary!
        let expected_cycles = 5;

        //when:
        let cycles_used = cpu.execute(expected_cycles, &mut mem);
    */

        let mut mem: Mem = Mem::new();
        let mut cpu = CPU::new();
        cpu.reset(&mut mem);

        //given:
        cpu.set_x(0x01);
        mem[0xFFFC] = CPU::INS_LDA_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x44; //0x4480
        mem[0x4481] = 0x37; 
        let expected_cycles = 4;
        let cpu_copy = cpu.clone();

        //when:
        let cycles_used = cpu.execute(expected_cycles, &mut mem);
}
