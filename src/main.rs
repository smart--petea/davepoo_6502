use davepoo_6502::{Mem, CPU, s32};

fn main() {
        //set up;
        let mut mem: Mem = Mem::new();
        let mut cpu = CPU::new();
        cpu.reset(&mut mem);

        //given:
        let NUM_CYCLES: s32 = 0;

        //when:
        let cycles_used: s32 = cpu.execute(NUM_CYCLES, &mut mem);

        //then:
        assert_eq!(cycles_used, 0);
}
