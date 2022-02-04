
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
        fn load_register(
            &mut self,
            address: Word,
            register_setter: fn(&mut CPU, u8),
            memory: &Mem,
            cycles: &mut s32,
        ) {
            let value: Byte = self.read_byte(cycles, address, memory);
            register_setter(self, value);
            self.load_register_set_status(value);
        }

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

        //L
        pub const INS_JSR: Byte = 0x20;

        //STA
        pub const INS_STA_ZP: Byte = 0x85;
        pub const INS_STA_ZPX: Byte = 0x95;
        pub const INS_STA_ABS: Byte = 0x8D;
        pub const INS_STA_ABSX: Byte = 0x9D;
        pub const INS_STA_ABSY: Byte = 0x99;
        pub const INS_STA_INDX: Byte = 0x81;
        pub const INS_STA_INDY: Byte = 0x91;

        //STX
        pub const INS_STX_ZP: Byte = 0x86;
        pub const INS_STX_ABS: Byte = 0x8E;

        //STY
        pub const INS_STY_ZP: Byte = 0x84;
        pub const INS_STY_ZPX: Byte = 0x94;
        pub const INS_STY_ABS: Byte = 0x8C;


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

        /** Addressing mode - Indirect X | Indexed Indirect */
        fn addr_indirect_x(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let mut zp_address: Word = self.fetch_byte(cycles, memory) as Word;
            zp_address = zp_address + self.x() as Word;
            *cycles = *cycles - 1;
            let effective_address: Word = self.read_word(cycles, zp_address, memory);
            effective_address
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

        /** Addressing mode - Indirect Y | Indirect Indexed */
        fn addr_indirect_y(&mut self, cycles: &mut s32, memory: &Mem) -> Word {
            let mut zp_address: Word = self.fetch_byte(cycles, memory) as Word;
            let effective_address: Word = self.read_word(cycles, zp_address, memory);
            let effective_address_y = effective_address + self.y() as Word;
            if effective_address_y - effective_address >= 0xFF {
                *cycles = *cycles - 1;
            }

            effective_address_y
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
                        self.load_register(address, CPU::set_y, memory, &mut cycles);
                    }
                    Self::INS_LDX_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        self.load_register(address, CPU::set_x, memory, &mut cycles);
                    }
                    Self::INS_LDX_ZPY => {
                        let address = self.addr_zero_page_y(&mut cycles, memory);
                        self.load_register(address, CPU::set_x, memory, &mut cycles);
                    }
                    Self::INS_LDA_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        self.load_register(address, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_LDY_ZPX => {
                        let mut address: Word = self.addr_zero_page_x(&mut cycles, memory);
                        self.load_register(address, CPU::set_y, memory, &mut cycles);
                    }
                    Self::INS_LDA_ZPX => {
                        let mut address: Word = self.addr_zero_page_x(&mut cycles, memory);
                        self.load_register(address, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_LDA_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        self.load_register(address, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_LDX_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        self.load_register(address, CPU::set_x, memory, &mut cycles);
                    }
                    Self::INS_LDY_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        self.load_register(address, CPU::set_y, memory, &mut cycles);
                    }
                    Self::INS_LDA_ABSX => {
                        let address = self.addr_absolute_x(&mut cycles, memory);
                        self.load_register(address, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_LDY_ABSX => {
                        let address = self.addr_absolute_x(&mut cycles, memory);
                        self.load_register(address, CPU::set_y, memory, &mut cycles);
                    }
                    Self::INS_LDA_ABSY => {
                        let address = self.addr_absolute_y(&mut cycles, memory);
                        self.load_register(address, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_LDX_ABSY => {
                        let address = self.addr_absolute_y(&mut cycles, memory);
                        self.load_register(address, CPU::set_x, memory, &mut cycles);
                    }
                    Self::INS_LDA_INDX => {
                        let effective_address: Word = self.addr_indirect_x(&mut cycles, memory);

                        self.load_register(effective_address, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_STA_INDX => {
                        let effective_address: Word = self.addr_indirect_x(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, effective_address, memory)
                    }
                    Self::INS_LDA_INDY => {
                        let effective_address_y = self.addr_indirect_y(&mut cycles, memory);
                        self.load_register(effective_address_y, CPU::set_a, memory, &mut cycles);
                    }
                    Self::INS_STA_INDY => {
                        let effective_address_y = self.addr_indirect_y(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, effective_address_y, memory);
                    }
                    Self::INS_STA_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, address, memory);

                    }
                    Self::INS_STX_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        self.write_byte(self.x(), &mut cycles, address, memory);

                    }
                    Self::INS_STY_ZP => {
                        let address = self.addr_zero_page(&mut cycles, memory);
                        self.write_byte(self.y(), &mut cycles, address, memory);

                    }
                    Self::INS_STA_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, address, memory);

                    }
                    Self::INS_STX_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        self.write_byte(self.x(), &mut cycles, address, memory);

                    }
                    Self::INS_STY_ABS => {
                        let address = self.addr_absolute(&mut cycles, memory);
                        self.write_byte(self.y(), &mut cycles, address, memory);

                    }
                    Self::INS_STA_ZPX => {
                        let address = self.addr_zero_page_x(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, address, memory);

                    }
                    Self::INS_STY_ZPX => {
                        let address = self.addr_zero_page_x(&mut cycles, memory);
                        self.write_byte(self.y(), &mut cycles, address, memory);

                    }
                    Self::INS_STA_ABSX => {
                        //TODO: AddAbsoluteX can consume an extra cycle on boundaries?
                        let address = self.addr_absolute_x(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, address, memory);

                        cycles = cycles - 1; //todo why the extra cycle is consumed
                    }
                    Self::INS_STA_ABSY => {
                        //TODO: AddAbsoluteY can consume an extra cycle on boundaries?
                        let address = self.addr_absolute_y(&mut cycles, memory);
                        self.write_byte(self.a(), &mut cycles, address, memory);

                        cycles = cycles - 1; //todo why the extra cycle is consumed
                    }
                    Self::INS_JSR => {
                        let sub_addr: Word = self.fetch_word(&mut cycles, memory);
                        self.write_word( self.sp(), &mut cycles, self.pc() - 1, memory,);

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
            let pc = self.pc();

            self.set_pc(pc + 2);
            *cycles = *cycles - 2;

            u16::from_le_bytes([memory[pc], memory[pc + 1]])
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

        /** write 1 byte to memory */
        fn write_byte(
            &self,
            value: Byte,
            cycles: &mut s32,
            address: Word,
            memory: &mut Mem,
        ) {
            memory[address] = value;
            *cycles = *cycles - 1;
        }

        /** write 2 bytes to memory */
        pub fn write_word(
            &mut self,
            value: Word,
            cycles: &mut s32,
            address: Word,
            memory: &mut Mem,
        ) {
            memory[address] = (value & 0xFF) as Byte;
            memory[address + 1] = (value >> 8) as Byte;

            *cycles = *cycles - 2;
        }
    }
}

#[cfg(test)]
mod tests;
