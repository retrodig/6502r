#[derive(Debug)]
pub struct StatusRegister {
    pub c: bool, // Carry Flag
    pub z: bool, // Zero Flag
    pub i: bool, // Interrupt Disable Flag
    pub d: bool, // Decimal Mode Flag
    pub b: bool, // Break Command Flag
    pub v: bool, // Overflow Flag
    pub n: bool, // Negative Flag
}

impl StatusRegister {
    pub fn new() -> Self {
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