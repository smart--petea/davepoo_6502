
pub mod m6502 {
    use modular_bitfield::*;
    use std::ops::{Index, IndexMut};

    pub type Byte = u8;
    pub type Word = u16;
    pub type s32 = i32;

    const MAX_MEM: usize = 1024 * 64;
    pub struct Mem
    {
        data: [Byte; MAX_MEM],
    }

    impl Mem {
        pub fn new() -> Self {
            Self {
                data: [Byte::default(); MAX_MEM],
            }
        }

        fn initialize(&mut self) {
            for data in self.data.iter_mut() {
                *data = Byte::default();
            }
        }
    }

    impl Mem {
        /** write 2 bytes */
        pub fn write_word(
            &mut self,
            value: Word,
            address: u16,
            cycles: &mut s32
        ) {
            let  address = address as usize;
            self.data[address] = (value & 0xFF) as Byte;
            self.data[address + 1] = (value >> 8) as Byte;

            *cycles = *cycles - 2;
        }
    }

    impl Index<u16> for Mem {
        type Output=Byte;

        fn index(&self, index: u16) -> &Byte {
            &self.data[index as usize]
        }
    }

    impl IndexMut<u16> for Mem {
        fn index_mut(&mut self, index: u16) -> &mut Self::Output {
            &mut self.data[index as usize]
        }
    }

    #[bitfield]
    #[derive(Debug, Clone)]
    pub struct CPU {
        pub pc: Word, //program counter
        pub sp: Word, //stack pointer

        pub a: Byte, //registers
        pub x: Byte, //registers
        pub y: Byte, //registers

        pub c: specifiers::B1, //status flag
        pub z: specifiers::B1, //status flag
        pub i: specifiers::B1, //status flag
        pub d: specifiers::B1, //status flag
        pub b: specifiers::B1, //status flag
        pub v: specifiers::B1, //status flag
        pub n: specifiers::B1, //status flag
        n2_dummy: specifiers::B1, //status flag
    }

    impl CPU {
        pub fn reset(&mut self, memory: &mut Mem)
        {
            self.set_pc(0xFFFC);
            self.set_sp(0x0100);
            self.set_d(0);
            self.set_a(0);
            self.set_x(0);
            self.set_y(0);

            self.set_c(0);
            self.set_z(0);
            self.set_i(0);
            self.set_b(0);
            self.set_v(0);
            self.set_n(0);

            memory.initialize();
        }

        //opcodes
        //LDA
        pub const INS_LDA_IM: Byte = 0xA9;
        pub const INS_LDA_ZP: Byte = 0xA5;
        pub const INS_LDA_ZPX: Byte = 0xB5;
        pub const INS_LDA_ABS: Byte = 0xAD;
        pub const INS_LDA_ABSX: Byte = 0xBD;
        pub const INS_LDA_ABSY: Byte = 0xB9;
        pub const INS_LDA_INDX: Byte = 0xA1; 
        pub const INS_LDA_INDY: Byte = 0xB1; 

        //LDX
        pub const INS_LDX_IM: Byte = 0xA2;
        pub const INS_LDX_ZP: Byte = 0xA6;
        pub const INS_LDX_ZPY: Byte = 0xB6;
        pub const INS_LDX_ABS: Byte = 0xAE;
        pub const INS_LDX_ABSY: Byte = 0xBE;

        //LDY
        pub const INS_LDY_IM: Byte = 0xA0;
        pub const INS_LDY_ZP: Byte = 0xA4;
        pub const INS_LDY_ZPX: Byte = 0xB4;
        pub const INS_LDY_ABS: Byte = 0xAC;
        pub const INS_LDY_ABSX: Byte = 0xBC;

        pub const INS_JSR: Byte = 0x20;

        /**Sets the correct Process status after a load register instruction
         * - LDA, LDY, LDZ
         * */
        fn load_register_set_status(&mut self, register: Byte) {
            self.set_z(if register == 0 {1} else {0});
            self.set_n(if register & 0b10000000 == 0 {0} else {1});
        }

        /**
         * Addressing mode - Zero page 
         */
        fn addr_zero_page(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let zero_page_address = self.fetch_byte(cycles, memory);
            zero_page_address as Word
        }

        /* Addresing mode - zero page with x offset */
        fn addr_zero_page_x(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let mut zero_page_address: Word = self.fetch_byte(cycles, memory) as Word;
            zero_page_address = (zero_page_address + self.x() as Word) & 0xFF;
            *cycles = *cycles - 1;

            zero_page_address
        }

        /* Addresing mode - zero page with y offset */
        fn addr_zero_page_y(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let mut zero_page_address: Word = self.fetch_byte(cycles, memory) as Word;
            zero_page_address = (zero_page_address + self.y() as Word) & 0xFF;
            *cycles = *cycles - 1;

            zero_page_address
        }

        /** Addressing mode - Absolute */
        fn addr_absolute(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let abs_address = self.fetch_word(cycles, memory);
            abs_address
        }

        /** Addressing mode - Absolute with X offset*/
        fn addr_absolute_x(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let mut abs_address = self.fetch_word(cycles, memory);
            let abs_address_x = abs_address + self.x() as Word;

            if abs_address_x - abs_address >= 0xFF {
                *cycles = *cycles - 1;
            }

            abs_address_x
        }

        /** Addressing mode - Absolute with Y offset*/
        fn addr_absolute_y(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let mut abs_address: Word = self.fetch_word(cycles, memory);
            let abs_address_y = abs_address + self.y() as Word;
            if abs_address_y - abs_address >= 0xFF {
                *cycles = *cycles - 1;
            }

            abs_address_y
        }

        //@return the number of cycles that were used
        pub fn execute(&mut self, cycles: s32, memory: &mut Mem) -> s32 {
            let cycles_requested = cycles;
            let mut cycles = cycles;
            while cycles > 0 {
                let ins: Byte = self.fetch_byte(&mut cycles, memory);

                match ins {
                    Self::INS_LDA_IM => {
                        let value: Byte = self.fetch_byte(&mut cycles, memory);
                        self.set_a(value);

                        self.load_register_set_status(value);
                    }
                    Self::INS_LDX_IM => {
                        let value: Byte = self.fetch_byte(&mut cycles, memory);
                        self.set_x(value);

                        self.load_register_set_status(value);
                    }
                    Self::INS_LDY_IM => {
                        let value: Byte = self.fetch_byte(&mut cycles, memory);
                        self.set_y(value);

                        self.load_register_set_status(value);
                    }
                    Self::INS_LDY_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_y(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDX_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_x(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDX_ZPY => {
                        let address = self.addr_zero_page_y(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_x(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_a(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDY_ZPX => {
                        let mut address: Word = self.addr_zero_page_x(&mut cycles, memory);

                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_y(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_ZPX => {
                        let mut address: Word = self.addr_zero_page_x(&mut cycles, memory);

                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_a(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_a(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDX_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_x(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDY_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_y(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_ABSX => {
                        let address = self.addr_absolute_x(&mut cycles, memory);

                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_a(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDY_ABSX => {
                        let address = self.addr_absolute_x(&mut cycles, memory);

                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_y(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_ABSY => {

                        let address = self.addr_absolute_y(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_a(value);

                        self.load_register_set_status(value);
                    }
                    Self::INS_LDX_ABSY => {

                        let address = self.addr_absolute_y(&mut cycles, memory);
                        let value: Byte = self.read_byte(&mut cycles, address, memory);
                        self.set_x(value);

                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_INDX => {
                        let mut zp_address: Word = self.fetch_byte(&mut cycles, memory) as Word;
                        zp_address = zp_address + self.x() as Word;
                        cycles = cycles - 1;
                        let effective_address: Word = self.read_word(&mut cycles, zp_address, memory);
                        let value: Byte = self.read_byte(&mut cycles, effective_address, memory);
                        self.set_a(value);
                        self.load_register_set_status(value);
                    }
                    Self::INS_LDA_INDY => {
                        let mut zp_address: Word = self.fetch_byte(&mut cycles, memory) as Word;
                        let effective_address: Word = self.read_word(&mut cycles, zp_address, memory);
                        let effective_address_y = effective_address + self.y() as Word;
                        let value: Byte = self.read_byte(&mut cycles, effective_address_y, memory);
                        self.set_a(value);
                        if effective_address_y - effective_address >= 0xFF {
                            cycles = cycles - 1;
                        }
                        self.load_register_set_status(value);
                    }
                    Self::INS_JSR => {
                        let sub_addr: Word = self.fetch_word(&mut cycles, memory);
                        memory.write_word(self.pc() - 1, self.sp(), &mut cycles);

                        self.set_pc(sub_addr);
                        cycles = cycles - 1;
                    }
                    _ => {
                        println!("Instruction not handled {}", ins);
                    }
                }
            }

            let num_cycles_used = cycles_requested - cycles;
            return num_cycles_used as s32;
        }

        fn fetch_word(
            &mut self,
            cycles: &mut s32,
            memory: &Mem
        ) -> Word {
            //6502 is little endian
            let mut data: Word = memory[self.pc()] as Word;
            self.set_pc(self.pc() + 1);

            let x = memory[self.pc()];
            let y = x as Word;
            let z = y << 8;
            data = data | z;
            //data = data | (( memory[self.pc()] as Word) << 8);
            self.set_pc(self.pc() + 1);

            *cycles = *cycles - 2;

            //if you wanted to handle endianess
            //you would have to swap bytes here
            //if (PLATFORM_BIG_ENDIAN)
            //  SwapBytesInWord()

            data
        }

        fn fetch_byte(
            &mut self,
            cycles: &mut s32,
            memory: &Mem
        ) -> Byte {
            let data: Byte = memory[self.pc()];
            self.set_pc(self.pc() + 1);
            *cycles = *cycles - 1;

            data
        }

        fn read_byte(
            &mut self,
            cycles: &mut s32,
            address: Word,
            memory: &Mem,
        ) -> Byte {
            let data: Byte = memory[address];
            *cycles = *cycles - 1;

            data
        }

        fn read_word(
            &mut self,
            cycles: &mut s32,
            address: Word,
            memory: &Mem,
        ) -> Word {
            let lo_byte = self.read_byte(cycles, address, memory) as Word;
            let hi_byte = self.read_byte(cycles, address + 1, memory) as Word;

            lo_byte | (hi_byte << 8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::m6502::*;

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
}
