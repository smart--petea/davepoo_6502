use davepoo_6502::m6502;

fn main() {
    test_load_register_immediate(m6502::CPU::INS_LDX_IM, m6502::CPU::x);
}

fn test_load_register_immediate(
    opcode_to_test: m6502::Byte,
    register_to_test: fn (&m6502::CPU) -> m6502::Byte,
) {
    let mut mem: m6502::Mem = m6502::Mem::new();
    let mut cpu = m6502::CPU::new();
    cpu.reset(&mut mem);

    //given:
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x84;

    //when:
    let cpu_copy = cpu.clone();
    let cycles_used = cpu.execute(2, &mut mem);

    //then:
    assert_eq!(cycles_used, 2);
    assert_eq!(register_to_test(&cpu), 0x84);
    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 1);

}
