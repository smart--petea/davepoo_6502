use crate::m6502::*;

macro_rules! verify_unmodified_flags_from_load_register {
    ($cpu:ident, $cpu_copy: ident) => {
        assert_eq!($cpu.c(), $cpu_copy.c());
        assert_eq!($cpu.b(), $cpu_copy.b());
        assert_eq!($cpu.d(), $cpu_copy.d());
        assert_eq!($cpu.i(), $cpu_copy.i());
        assert_eq!($cpu.v(), $cpu_copy.v());
    };
}

fn test_load_register_immediate(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_z(1);
    cpu.set_n(0);
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

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

fn test_load_register_zero_page(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_z(1);
    cpu.set_n(1);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0x37;

    //when:
    let cpu_copy = cpu.clone();
    let cycles_used = cpu.execute(3, &mut mem);

    //then:
    assert_eq!(cycles_used, 3);

    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

#[test]
fn lda_immediate_can_load_a_value_into_the_a_register() {
    test_load_register_immediate(CPU::INS_LDA_IM, CPU::a);
}

#[test]
fn ldx_immediate_can_load_a_value_into_the_x_register() {
    test_load_register_immediate(CPU::INS_LDX_IM, CPU::x);
}

#[test]
fn ldy_immediate_can_load_a_value_into_the_y_register() {
    test_load_register_immediate(CPU::INS_LDY_IM, CPU::y);
}

#[test]
fn lda_immediate_can_affect_the_zero_flag() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_a(0x44);
    mem[0xFFFC] = CPU::INS_LDA_IM;
    mem[0xFFFD] = 0x0;

    //when:
    let cpu_copy = cpu.clone();
    cpu.execute(2, &mut mem);

    //then:
    assert_eq!(cpu.a(), 0x0);
    assert_eq!(cpu.z(), 1);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn lda_zero_page_can_load_a_value_into_the_a_register() {
    test_load_register_zero_page(CPU::INS_LDA_ZP, CPU::a);
}

#[test]
fn ldx_zero_page_can_load_a_value_into_the_x_register() {
    test_load_register_zero_page(CPU::INS_LDX_ZP, CPU::x);
}

#[test]
fn ldy_zero_page_can_load_a_value_into_the_y_register() {
    test_load_register_zero_page(CPU::INS_LDY_ZP, CPU::y);
}

fn test_load_register_zero_page_y(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_y(5);

    //start - inline a little program
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x42;
    mem[0x0047] = 0x37;
    //end - inline a little program

    //when:
    let cpu_copy = cpu.clone();
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    assert_eq!(cycles_used, 4);

    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

fn test_load_register_zero_page_x(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_z(1);
    cpu.set_n(1);
    cpu.set_x(5);

    //start - inline a little program
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x42;
    mem[0x0047] = 0x37;
    //end - inline a little program

    //when:
    let cpu_copy = cpu.clone();
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    assert_eq!(cycles_used, 4);

    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

#[test]
fn lda_zero_page_x_can_load_a_value_into_the_a_register() {
    test_load_register_zero_page_x(CPU::INS_LDA_ZPX, CPU::a);
}

#[test]
fn ldx_zero_page_y_can_load_a_value_into_the_x_register() {
    test_load_register_zero_page_y(CPU::INS_LDX_ZPY, CPU::x);
}

#[test]
fn ldy_zero_page_x_can_load_a_value_into_the_y_register() {
    test_load_register_zero_page_x(CPU::INS_LDY_ZPX, CPU::y);
}

#[test]
fn the_cpu_does_nothing_when_we_execute_zero_cycles() {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    let num_cycles: s32 = 0;

    //when:
    let cycles_used: s32 = cpu.execute(num_cycles, &mut mem);

    //then:
    assert_eq!(cycles_used, 0);
}

#[test]
fn cpu_can_execute_more_cycles_than_requested_if_required_by_instructions () {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    mem[0xFFFC] = CPU::INS_LDA_IM;
    mem[0xFFFD] = 0x84;

    //when:
    let cycles_used: s32 = cpu.execute(1, &mut mem);

    //then:
    assert_eq!(cycles_used, 2);
}

#[test]
fn lda_zero_page_x_can_load_a_value_into_the_a_register_when_it_wraps() {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_x(0xFF);


    //start - inline a little program
    mem[0xFFFC] = CPU::INS_LDA_ZPX;
    mem[0xFFFD] = 0x80;
    mem[0x007F] = 0x37;
    //end - inline a little program

    //when:
    let cpu_copy = cpu.clone();
    let cycles_used = cpu.execute(4, &mut mem);

    //then:
    assert_eq!(cycles_used, 4);

    assert_eq!(cpu.a(), 0x37);
    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

fn test_load_register_absolute(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_z(1);
    cpu.set_n(1);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4480] = 0x37; 
    let expected_cycles = 4;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(cycles_used, expected_cycles);
    assert_eq!(register_to_test(&cpu), 0x37);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

#[test]
fn lda_absolute_can_load_a_value_into_the_a_register() {
    test_load_register_absolute(CPU::INS_LDA_ABS, CPU::a);
}

#[test]
fn ldx_absolute_can_load_a_value_into_the_x_register() {
    test_load_register_absolute(CPU::INS_LDX_ABS, CPU::x);
}

#[test]
fn ldy_absolute_can_load_a_value_into_the_y_register() {
    test_load_register_absolute(CPU::INS_LDY_ABS, CPU::y);
}

fn test_load_register_absolute_x(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_z(1);
    cpu.set_n(1);
    cpu.set_x(0x01);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4481] = 0x37; 
    let expected_cycles = 4;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

fn test_load_register_absolute_y(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_z(1);
    cpu.set_n(1);
    cpu.set_y(0x01);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x80;
    mem[0xFFFE] = 0x44; //0x4480
    mem[0x4481] = 0x37; 
    let expected_cycles = 4;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

#[test]
fn lda_absolute_x_can_load_a_value_into_the_a_register() {
    test_load_register_absolute_x(CPU::INS_LDA_ABSX, CPU::a);
}

#[test]
fn ldx_absolute_y_can_load_a_value_into_the_x_register() {
    test_load_register_absolute_y(CPU::INS_LDX_ABSY, CPU::x);
}

#[test]
fn ldy_absolute_x_can_load_a_value_into_the_y_register() {
    test_load_register_absolute_x(CPU::INS_LDY_ABSX, CPU::y);
}

fn test_load_register_absolute_x_when_crossing_page(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_x(0xFF);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x02;
    mem[0xFFFE] = 0x44; //0x4402
    mem[0x4501] = 0x37; //0x4402+0xFF crosses page boundary!
    let expected_cycles = 5;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

#[test]
fn lda_absolute_x_can_load_a_value_into_the_a_register_when_it_crosses_a_page_boundary() {
    test_load_register_absolute_x_when_crossing_page(CPU::INS_LDA_ABSX, CPU::a);
}

#[test]
fn ldy_absolute_x_can_load_a_value_into_the_y_register_when_it_crosses_a_page_boundary() {
    test_load_register_absolute_x_when_crossing_page(CPU::INS_LDY_ABSX, CPU::y);
}

#[test]
fn lda_absolute_y_can_load_a_value_into_the_a_register() {
    test_load_register_absolute_y(CPU::INS_LDA_ABSY, CPU::a);
}

fn test_load_register_absolute_y_when_crossing_page(
    opcode_to_test: Byte,
    register_to_test: fn (&CPU) -> Byte,
) {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_y(0xFF);
    mem[0xFFFC] = opcode_to_test;
    mem[0xFFFD] = 0x02;
    mem[0xFFFE] = 0x44; //0x4402
    mem[0x4501] = 0x37; //0x4402+0xFF crosses page boundary!
    let expected_cycles = 5;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(register_to_test(&cpu), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
    }

#[test]
fn lda_absolute_y_can_load_a_value_into_the_a_register_when_it_crosses_a_a_page_boundary() {
    test_load_register_absolute_y_when_crossing_page(CPU::INS_LDA_ABSY, CPU::a);
}

#[test]
fn ldx_absolute_y_can_load_a_value_into_the_x_register_when_it_crosses_a_a_page_boundary() {
    test_load_register_absolute_y_when_crossing_page(CPU::INS_LDX_ABSY, CPU::x);
}

#[test]
fn lda_indirect_x_can_load_a_value_into_the_a_register() {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_x(0x04);
    mem[0xFFFC] = CPU::INS_LDA_INDX;
    mem[0xFFFD] = 0x02;
    mem[0x0006] = 0x00; //0x2 + 0x4
    mem[0x0007] = 0x80; 
    mem[0x8000] = 0x37; 
    let expected_cycles = 6;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(cpu.a(), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_a_value_into_the_a_register() {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_y(0x04);
    mem[0xFFFC] = CPU::INS_LDA_INDY;
    mem[0xFFFD] = 0x02;
    mem[0x0002] = 0x00; 
    mem[0x0003] = 0x80; 
    mem[0x8004] = 0x37; //0x8000 + 0x4
    let expected_cycles = 5;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(cpu.a(), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_a_value_into_the_a_register_when_it_crosses_a_page() {
    //set up;
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);

    //given:
    cpu.set_y(0xFF);
    mem[0xFFFC] = CPU::INS_LDA_INDY;
    mem[0xFFFD] = 0x02;
    mem[0x0002] = 0x02; 
    mem[0x0003] = 0x80; 
    mem[0x8101] = 0x37; //0x8002 + 0xFF
    let expected_cycles = 6;
    let cpu_copy = cpu.clone();

    //when:
    let cycles_used = cpu.execute(expected_cycles, &mut mem);

    //then:
    assert_eq!(cpu.a(), 0x37);
    assert_eq!(cycles_used, expected_cycles);

    assert_eq!(cpu.z(), 0);
    assert_eq!(cpu.n(), 0);

    verify_unmodified_flags_from_load_register!(cpu, cpu_copy);
}
