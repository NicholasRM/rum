struct Field {
    lsb: u32,
    width: u32,
}

static OPCODE:Field = Field{
    lsb: 28,
    width: 4
};

static REG_A:Field = Field{
    lsb: 6,
    width: 3
};

static REG_B:Field = Field{
    lsb: 3,
    width: 3
};

static REG_C:Field = Field{
    lsb: 0,
    width: 3
};

static LOAD_REG:Field = Field{
    lsb: 25,
    width: 3
};

static LOAD_VAL:Field = Field{
    lsb: 0,
    width: 25
};

pub fn get_opcode(instruction: u32) -> u32 {
    get(&OPCODE, instruction)
}

pub fn get_one_reg(instruction: u32) -> usize {
    get(&REG_C, instruction) as usize
}

pub fn get_two_regs(instruction: u32) -> (usize, usize) {
    (get(&REG_B, instruction) as usize, get(&REG_C, instruction) as usize)
}

pub fn get_three_regs(instruction: u32) -> (usize, usize, usize) {
    (get(&REG_A, instruction) as usize, get(&REG_B, instruction) as usize, get(&REG_C, instruction) as usize)
}

pub fn get_load_reg(instruction: u32) -> usize {
    get(&LOAD_REG, instruction) as usize
}

pub fn get_value(instruction: u32) -> u32 {
    get(&LOAD_VAL, instruction)
}

fn get(field: &Field, instruction: u32) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}