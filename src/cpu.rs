use crate::memory::Memory;
use crate::register::StatusRegister;

struct CPU {
    a: u8,  // A Register
    x: u8,  // X Register
    y: u8,  // Y Register
    pc: u16, // Program Counter
    sp: u8,  // Stack Pointer
    status: StatusRegister, // Status Register
}

impl CPU {
    fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFF, // Initial value of stack pointer is 0xFF
            status: StatusRegister::new(),
        }
    }

    fn execute(&mut self, opcode: u8, memory: &mut Memory) {
        let opcode = memory.read(self.pc); // Reads the instruction code from the address pointed to by the PC
        self.pc += 1; // Increment PC

        match opcode {
            0x00 => { // BRK Instruction
                self.status.b = true; // Set Break Command flag
                // interrupt processing
                // ...
            },
            0x01 => { // ORA ($NN,X)
                let addr = self.indirect_x_addr(memory);
                let data = memory.read(addr);
                self.ora(data);
            },
            0x69 => { // ADC #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.adc(immediate);
            },
            0xA9 => { // LDA #$NN
                let immediate = memory.read(self.pc);
                self.pc += 1;
                self.lda(immediate);
            },
            _ => panic!("Unimplemented instruction code: {:02X}", opcode),
        }
    }

    // --- Address acquisition functions for various addressing modes ---
    fn zero_page_addr(&self, memory: &Memory) -> u16 {
        memory.read(self.pc) as u16
    }

    fn zero_page_x_addr(&self, memory: &Memory) -> u16 {
        (memory.read(self.pc) as u16 + self.x as u16) & 0xFF
    }

    fn zero_page_y_addr(&self, memory: &Memory) -> u16 {
        (memory.read(self.pc) as u16 + self.y as u16) & 0xFF
    }

    fn absolute_addr(&self, memory: &Memory) -> u16 {
        let lo = memory.read(self.pc) as u16;
        let hi = memory.read(self.pc + 1) as u16;
        (hi << 8) | lo
    }

    fn absolute_x_addr(&self, memory: &Memory) -> u16 {
        (self.absolute_addr(memory) + self.x as u16)
    }

    fn absolute_y_addr(&self, memory: &Memory) -> u16 {
        (self.absolute_addr(memory) + self.y as u16)
    }

    fn indirect_addr(&self, memory: &Memory) -> u16 {
        let lo_addr = memory.read(self.pc) as u16;
        let hi_addr = memory.read(self.pc + 1) as u16;
        let lo = memory.read(lo_addr) as u16;
        let hi = memory.read((hi_addr << 8) | ((lo_addr + 1) & 0xFF)) as u16;
        (hi << 8) | lo
    }

    fn indirect_x_addr(&self, memory: &Memory) -> u16 {
        let addr = (memory.read(self.pc) as u16 + self.x as u16) & 0xFF;
        let lo = memory.read(addr) as u16;
        let hi = memory.read((addr + 1) & 0xFF) as u16;
        (hi << 8) | lo
    }

    fn indirect_y_addr(&self, memory: &Memory) -> u16 {
        let addr = memory.read(self.pc) as u16;
        let lo = memory.read(addr) as u16;
        let hi = memory.read((addr + 1) & 0xFF) as u16;
        (hi << 8) | lo + self.y as u16
    }

    // --- Implementation of various instructions ---
    fn lda(&mut self, value: u8) {
        self.a = value;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn adc(&mut self, value: u8) {
        let sum = self.a as u16 + value as u16 + self.status.c as u16;
        self.status.c = sum > 0xFF;
        self.status.v = ((self.a ^ value) & 0x80 == 0) && ((self.a as u16 ^ sum) & 0x80 != 0);
        self.a = (sum & 0xFF) as u8;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }

    fn ora(&mut self, value: u8) {
        self.a |= value;
        self.status.z = self.a == 0;
        self.status.n = self.a & 0x80 != 0;
    }
}
