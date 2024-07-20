struct CPU {
    a: u8,  // A Register
    x: u8,  // X Register
    y: u8,  // Y Register
    pc: u16, // Program Counter
    sp: u8,  // Stack Pointer
    status: StatusRegister, // Status Register
}

struct StatusRegister {
    c: bool, // Carry Flag
    z: bool, // Zero Flag
    i: bool, // Interrupt Disable Flag
    d: bool, // Decimal Mode Flag
    b: bool, // Break Command Flag
    v: bool, // Overflow Flag
    n: bool, // Negative Flag
}

struct Memory {
    data: [u8; 0x10000], // 65,536 bytes of memory
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
    }
}

impl Memory {
    fn new() -> Self {
        Memory { data: [0; 0x10000] }
    }

    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }
}

impl StatusRegister {
    fn new() -> Self {
        StatusRegister {
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }
}
