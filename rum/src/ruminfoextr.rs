/// A struct representing a valid bitfield in an instruction.
/// This struct, and all additional code, was adapted from the `rumdump`
/// lab and was provided by Noah Daniels
struct Field {
    lsb: u32,
    width: u32,
}

/// The field representing the opcode
static OPCODE:Field = Field{
    lsb: 28,
    width: 4
};

/// The field representing register A in a three register instruction
static REG_A:Field = Field{
    lsb: 6,
    width: 3
};

/// The field representing register B in a three register instruction
static REG_B:Field = Field{
    lsb: 3,
    width: 3
};

/// The field representing register C in a three register instruction
static REG_C:Field = Field{
    lsb: 0,
    width: 3
};

/// The field representing the load register in a load instruction
static LOAD_REG:Field = Field{
    lsb: 25,
    width: 3
};

/// The field representing the literal value in a load instruction
static LOAD_VAL:Field = Field{
    lsb: 0,
    width: 25
};

/// Function to get the opcode of an instruction
/// 
/// Returns the opcode as a `u32`
/// 
/// # Arguments:
/// * `instruction`: the current instruction being executed
pub fn get_opcode(instruction: u32) -> u32 {
    get(&OPCODE, instruction)
}

/// Function to get the C register from an instruction
/// 
/// Returns the register value as a `usize`
/// 
/// # Arguments:
/// * `instruction`: the current instruction being executed
pub fn get_one_reg(instruction: u32) -> usize {
    get(&REG_C, instruction) as usize
}

/// Function to get the B and C registers from an instruction
/// 
/// Returns the registers as `usize`s contained in a tuple
/// 
/// # Arguments:
/// * `instruction`: the current instruction being executed
pub fn get_two_regs(instruction: u32) -> (usize, usize) {
    (get(&REG_B, instruction) as usize, get(&REG_C, instruction) as usize)
}


/// Function to get the A, B, and C registers from an instruction
/// 
/// Returns the registers as a `usizes` contained in a tuple
/// 
/// # Arguments:
/// * `instruction`: the current instruction being executed
pub fn get_three_regs(instruction: u32) -> (usize, usize, usize) {
    (get(&REG_A, instruction) as usize, get(&REG_B, instruction) as usize, get(&REG_C, instruction) as usize)
}

/// Function to get the load register from an instruction
/// 
/// Returns the register as a `usize`
/// 
/// # Arguments:
/// * `instruction`: the current instruction being executed
pub fn get_load_reg(instruction: u32) -> usize {
    get(&LOAD_REG, instruction) as usize
}

/// Function to get the value from an instruction
/// 
/// Returns the value as a `u32`
/// 
/// # Arguments:
/// * `instruction`: the current instruction being executed
pub fn get_value(instruction: u32) -> u32 {
    get(&LOAD_VAL, instruction)
}

/// Helper function to get a value at a given part of an instruction
/// 
/// Returns the value as a `u32`
/// 
/// # Arguments:
/// * `field`: A reference to one of the fields defined previously
/// * `instruction`: the current instruction being executed
fn get(field: &Field, instruction: u32) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Helper function to create a bit mask used in extracting information from the instruction
/// 
/// Returns the mask as a `u32`
/// 
/// # Arguments:
/// * `bits`: The number of bits that will be on, starting from the least significant bit
fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}