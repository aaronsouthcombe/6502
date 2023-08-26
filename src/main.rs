type Byte = u8;
type Word = u16;

// Define the struct for the Processor Status, to be used in the status CPU struct field
struct PS {
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    overflow: bool,
    negative: bool,
}

// Implement an instantiation method thing
impl PS {
    fn new() -> Self {
        PS {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }
}

// Define the struct for the actual CPU with registers and flags
struct CPU {
    pc: Word, // Program Counter
    sp: Word, // Stack Pointer

    a: Byte, // Accumulator
    x: Byte, // Index x
    y: Byte, // Index y (both registers)

    status: PS, // Processor Status, fields are autodescriptive in the struct, reference
                // documentation for the 6502 if unclear
}

// Implement instantiation method
impl CPU {
    fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            status: PS::new(),
        }
    }
}



// Kinda obvious, but the execution section
fn main() {
    let cpu = CPU::new();
}
