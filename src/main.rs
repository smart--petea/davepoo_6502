use modular_bitfield::*;

type Byte = u8;
type Word = u16;

const MAX_MEM: usize = 1024 * 64;
struct Mem
{
    Data: [Byte; MAX_MEM],
}

impl Mem {
    fn new() -> Self {
        Self {
            Data: [Byte::default(); MAX_MEM],
        }
    }

    fn initialize(&mut self) {
        for data in self.Data.iter_mut() {
            *data = Byte::default();
        }
    }

}

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
    fn reset(&mut self, memory: &mut Mem)
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

        memory.initialize();
    }

    fn execute(&self, Ticks: u32, mem: &Mem) {
    }
}

fn main() {
    let mut mem: Mem = Mem::new();
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);
    cpu.execute(2, &mut mem);
}
