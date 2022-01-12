use modular_bitfield::*;
use std::ops::{Index, IndexMut};

type Byte = u8;
type Word = u16;

const MAX_MEM: usize = 1024 * 64;
struct Mem
{
    data: [Byte; MAX_MEM],
}

impl Mem {
    fn new() -> Self {
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
struct CPU {
    pc: Word, //program counter
    sp: Word, //stack pointer

    a: Byte, //registers
    x: Byte, //registers
    y: Byte, //registers

    c: specifiers::B1, //status flag
    z: specifiers::B1, //status flag
    i: specifiers::B1, //status flag
    d: specifiers::B1, //status flag
    b: specifiers::B1, //status flag
    v: specifiers::B1, //status flag
    n: specifiers::B1, //status flag
    n2_dummy: specifiers::B1, //status flag
}

impl CPU {
    fn reset(&mut self, memory: &mut Mem)
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
    const INS_LDA_IM: Byte = 0xA9;
    const INS_LDA_ZP: Byte = 0xA5;
    const INS_LDA_ZPX: Byte = 0xB5;

    fn LDA_set_status(&mut self) {
        let a = self.a();

        self.set_z(if a == 0 {1} else {0});
        self.set_n(if a & 0b10000000 == 0 {0} else {1});
    }

    fn execute(&mut self, cycles: u32, memory: &Mem) {
        let mut cycles = cycles;
        while cycles > 0u32 {
            let ins: Byte = self.fetch_byte(&mut cycles, memory);

            match ins {
                Self::INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.set_a(value);
                    self.LDA_set_status();
                }
                Self::INS_LDA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    let value: Byte = self.read_byte(&mut cycles, zero_page_address, memory);
                    self.set_a(value);
                    self.LDA_set_status();
                }
                Self::INS_LDA_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    zero_page_address = zero_page_address + self.x();
                    cycles = cycles - 1;

                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.set_a(value);
                    self.LDA_set_status();
                }
                _ => {
                    unimplemented!("Instruction not handled {}", ins);
                }
            }
        }
    }

    fn fetch_word(&mut self, cycles: &mut u32, memory: &Mem) -> Word {
        //6502 is little endian
        let mut data: Word = memory[self.pc()] as Word;
        self.set_pc(self.pc() + 1);

        data = data | ((memory[self.pc()] as Word) << 8);
        self.set_pc(self.pc() + 1);

        *cycles = *cycles - 2;

        //if you wanted to handle endianess
        //you would have to swap bytes here
        //if (PLATFORM_BIG_ENDIAN)
        //  SwapBytesInWord()

        data
    }

    fn fetch_byte(&mut self, cycles: &mut u32, memory: &Mem) -> Byte {
        let data: Byte = memory[self.pc()];
        self.set_pc(self.pc() + 1);
        *cycles = *cycles - 1;

        data
    }

    fn read_byte(
        &mut self,
        cycles: &mut u32,
        address: Byte,
        memory: &Mem,
    ) -> Byte {
        let data: Byte = memory[address as u16];
        *cycles = *cycles - 1;

        data
    }
}

fn main() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);
    //start - inline a little program
    mem[0xFFFC] = CPU::INS_LDA_ZP;
    mem[0xFFFD] = 0x42;
    mem[0x0042] = 0x84;
    //end - inline a little program
    cpu.execute(3, &mut mem);
    mem[0x0042] = 0x84;
}
