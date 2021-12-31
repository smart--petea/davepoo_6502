use modular_bitfield::*;

type Byte = u8;
type Word = u16;

#[bitfield]
struct CPU {
    PC: Word, //program counter
    SP: Word, //stack pointer

    A: Byte, //registers
    X: Byte, //registers
    Y: Byte, //registers

    C: specifiers::B1, //status flag
    Z: specifiers::B1, //status flag
    I: specifiers::B1, //status flag
    D: specifiers::B1, //status flag
    B: specifiers::B1, //status flag
    V: specifiers::B1, //status flag
    N: specifiers::B1, //status flag
    N2Dummy: specifiers::B1, //status flag
}

impl CPU {
    fn reset(&mut self)
    {
        self.set_PC(0xFFFC);
        self.set_SP(0x0100);
        self.set_D(0);
        self.set_A(0);
        self.set_X(0);
        self.set_Y(0);

        self.set_C(0);
        self.set_Z(0);
        self.set_I(0);
        self.set_B(0);
        self.set_V(0);
        self.set_N(0);
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.reset();
}
