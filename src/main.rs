use davepoo_6502::m6502;

fn main() {
    test_store_register_zero_page(m6502::CPU::INS_STA_ZP, m6502::CPU::set_a);
}

fn test_store_register_zero_page(
    opcode_to_test: m6502::Byte,
    register_to_test: fn(&mut m6502::CPU, m6502::Byte),
) {
    let mut mem: m6502::Mem = m6502::Mem::new();
    let mut cpu = m6502::CPU::new();
    cpu.reset(&mut mem);

    //given:
    register_to_test(&mut cpu, 0x2F);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0x0080] = 0x00;
    const EXPECTED_CYCLES: m6502::s32 = 3;

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0080], 0x2F);
}
