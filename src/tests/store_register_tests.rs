use crate::m6502::*;

macro_rules! verify_unmodified_flags_from_load_register {
    ($cpu:ident, $cpu_copy: ident) => {
        assert_eq!($cpu.c(), $cpu_copy.c());
        assert_eq!($cpu.b(), $cpu_copy.b());
        assert_eq!($cpu.d(), $cpu_copy.d());
        assert_eq!($cpu.i(), $cpu_copy.i());
        assert_eq!($cpu.v(), $cpu_copy.v());
        assert_eq!($cpu.n(), $cpu_copy.n());
    };
}

fn test_store_register_zero_page(
    opcode_to_test: Byte,
    register_to_test: fn(&mut CPU, Byte),
) {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    register_to_test(&mut cpu, 0x2F);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0x0080] = 0x00;
    const EXPECTED_CYCLES: s32 = 3;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, EXPECTED_CYCLES);
    assert_eq!(mem[0x0080], 0x2F);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

fn test_store_register_absolute(
    opcode_to_test: Byte,
    register_to_test: fn(&mut CPU, Byte),
) {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    register_to_test(&mut cpu, 0x2F);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x8000] = 0x00;
    const EXPECTED_CYCLES: s32 = 4;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x8000], 0x2F);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

fn test_store_register_zero_page_x(
    opcode_to_test: Byte,
    register_to_test: fn(&mut CPU, Byte),
) {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_x(0x0F);
    register_to_test(&mut cpu, 0x42);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0x008F] = 0x00;
    const EXPECTED_CYCLES: s32 = 4;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x008F], 0x42);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn sta_zero_page_can_store_the_a_register_into_memory() {
    test_store_register_zero_page(CPU::INS_STA_ZP, CPU::set_a);
}

#[test]
fn stx_zero_page_can_store_the_x_register_into_memory() {
    test_store_register_zero_page(CPU::INS_STX_ZP, CPU::set_x);
}

#[test]
fn sty_zero_page_can_store_the_y_register_into_memory() {
    test_store_register_zero_page(CPU::INS_STY_ZP, CPU::set_y);
}

#[test]
fn sta_absolute_can_store_the_a_register_into_memory() {
    test_store_register_absolute(CPU::INS_STA_ABS, CPU::set_a);
}

#[test]
fn stx_absolute_can_store_the_x_register_into_memory() {
    test_store_register_absolute(CPU::INS_STX_ABS, CPU::set_x);
}

#[test]
fn sty_absolute_can_store_the_y_register_into_memory() {
    test_store_register_absolute(CPU::INS_STY_ABS, CPU::set_y);
}

#[test]
fn sta_zero_page_x_can_store_the_a_register_into_memory() {
    test_store_register_zero_page_x(CPU::INS_STA_ZPX, CPU::set_a);
}

#[test]
fn sty_zero_page_x_can_store_the_y_register_into_memory() {
    test_store_register_zero_page_x(CPU::INS_STY_ZPX, CPU::set_y);
}

#[test]
fn sta_absolute_x_can_store_the_register_into_memory() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_x(0x0F);
    cpu.set_a(0x42);
    mem[0xFFFC] = CPU::INS_STA_ABSX;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x800F] = 0x00;
    const EXPECTED_CYCLES: s32 = 5;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x800F], 0x42);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn sta_absolute_y_can_store_the_register_into_memory() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_y(0x0F);
    cpu.set_a(0x42);
    mem[0xFFFC] = CPU::INS_STA_ABSY;
    mem[0xFFFD] = 0x00;
    mem[0xFFFE] = 0x80;
    mem[0x800F] = 0x00;
    const EXPECTED_CYCLES: s32 = 5;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x800F], 0x42);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn sta_indirect_x_can_store_the_register_into_memory() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_x(0x0F);
    cpu.set_a(0x42);
    mem[0xFFFC] = CPU::INS_STA_INDX;
    mem[0xFFFD] = 0x20;
    mem[0x002F] = 0x00;
    mem[0x0030] = 0x80;
    mem[0x8000] = 0x00;
    const EXPECTED_CYCLES: s32 = 6;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x8000], 0x42);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn sta_indirect_y_can_store_the_register_into_memory() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_y(0x0F);
    cpu.set_a(0x42);
    mem[0xFFFC] = CPU::INS_STA_INDY;
    mem[0xFFFD] = 0x20;
    mem[0x0020] = 0x00;
    mem[0x0021] = 0x80;
    mem[0x8000 + 0x0F] = 0x00;
    const EXPECTED_CYCLES: s32 = 6;
    let cpu_copy = cpu.clone();

    //when:
    let actual_cycles = cpu.execute(EXPECTED_CYCLES, &mut mem);

    //then:
    assert_eq!(actual_cycles, actual_cycles);
    assert_eq!(mem[0x8000 + 0x0F], 0x42);
    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}
