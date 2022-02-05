use crate::m6502::*;

#[test]
fn can_jump_to_subroutine_and_jump_back_again() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(0xFF00, &mut mem);

    //given:
    mem[0xFF00] = CPU::INS_JSR;
    mem[0xFF01] = 0x00;
    mem[0xFF02] = 0x80;
    mem[0x8000] = CPU::INS_RTS;
    mem[0xFF03] = CPU::INS_LDA_IM;
    mem[0xFF04] = 0x42;

    //6 cycles for CPU::INS_JSR
    //6 cycles for CPU::INS_RTS
    //2 cycles for CPU::INS_LDA_IM
    const EXPECTED_CYCLES: s32 = 6 + 6;

    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(cpu.a(), 0x42);
}
