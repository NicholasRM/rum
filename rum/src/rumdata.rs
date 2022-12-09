use core::panic;

use crate::ruminfoextr::{get_three_regs, get_one_reg, get_load_reg, get_value};
use crate::{rumcpu::RumCpu, ruminfoextr::get_opcode};
use crate::rummemory::RumMemory;
use std::io::{Read, self};

enum Opcode {
    CMov,
    SegLoad,
    SegStore,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    Map,
    Unmap,
    Output,
    Input,
    LoadProgram,
    LoadValue
}

pub struct RumData {
    pub cpu: RumCpu,
    pub memory: RumMemory,
}

impl RumData {
    pub fn load(program: Vec<u32>) -> Self {
        RumData { cpu: RumCpu::init(), memory: RumMemory::init(program) }
    }

    pub fn execute(&mut self) {
        let mut instruction = 0;//get it from seg0
        loop{
            instruction = self.memory.get_seg_val(0, self.cpu.pc);
            match get_opcode(instruction){
                0 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.cmov(reg_a, reg_b, reg_c)}
                1 => {

                }
                2 => {

                }
                3 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.add(reg_a, reg_b, reg_c)}
                4 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.mul(reg_a, reg_b, reg_c)}
                5 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.div(reg_a, reg_b, reg_c)}
                6 => {
                    let (reg_a, reg_b, reg_c) = get_three_regs(instruction);
                    self.cpu.nand(reg_a, reg_b, reg_c)}
                7 => {
                    std::process::exit(0);
                }
                8 => {

                }
                9 => {

                }
                10 => {
                    let reg_c = get_one_reg(instruction);
                    self.output(reg_c);
                }
                11 => {
                    let reg_c = get_one_reg(instruction);
                    self.input(reg_c);
                }
                12 => {

                }
                13 => {
                    let load_reg = get_load_reg(instruction);
                    let value = get_value(instruction);
                    self.cpu.load_value(load_reg, value);
                }
                _ => {
                    panic!("invalid instruction");
                }
            }
        }
    }
    
    pub fn output(&self, reg_c: usize) {
        if self.cpu.regs[reg_c] > 255{
            panic!("Value too big");
        }
        print!("{}", self.cpu.regs[reg_c] as u8 as char);
    }

    pub fn input(&mut self, reg_c: usize) {
        let mut buffer: [u8; 1] = [0];
        let input = io::stdin().read(&mut buffer);
        match input.unwrap(){
            0 => self.cpu.load_value(reg_c, u32::MAX),
            1 => self.cpu.load_value(reg_c, buffer[0] as u32),
            _ => panic!("Too many bites in buffer"),
        }
    }
}
