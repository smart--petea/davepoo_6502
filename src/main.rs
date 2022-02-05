use davepoo_6502::m6502;

fn main() {
    let mut mem: m6502::Mem = m6502::Mem::new();
    let mut cpu = m6502::CPU::new();
    cpu.reset(0xFFFC, &mut mem);

    //given:
    cpu.set_x(0x0F);
    cpu.set_a(0x42);
    mem[0xFFFC] = m6502::CPU::INS_STA_ABSX;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x800F] = 0x00;
    const EXPECTED_CYCLES: m6502::s32 = 5;

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x800F], 0x42);

}
