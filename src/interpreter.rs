/*
 Copyright (c) 2022 Tor Ludwig Bogsveen

 Permission is hereby granted, free of charge, to any person obtaining a copy of
 this software and associated documentation files (the "Software"), to deal in
 the Software without restriction, including without limitation the rights to
 use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 the Software, and to permit persons to whom the Software is furnished to do so,
 subject to the following conditions:

 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::{panic, ops::{Index, IndexMut}, fmt::Display};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum CompilerCall {
    None,
    PrintInt,
}

impl From<&str> for CompilerCall {
    fn from(call: &str) -> Self {
        match call.trim() {
            "print_int" => CompilerCall::PrintInt,
            _ => CompilerCall::None,
        }
    }
}

impl From<u8> for CompilerCall {
    fn from(call: u8) -> Self {
        match call {
            1 => Self::PrintInt,
            _ => Self::None,
        }
    }
}

impl From<CompilerCall> for u8 {
    fn from(call: CompilerCall) -> Self {
        unsafe {
            std::mem::transmute(call)
        }
    }
}

pub struct Flags {
    pub not_zero: bool,
    pub less_then: bool,
    pub larger_then: bool,
    pub equals: bool,
    pub overflow: bool,
    pub underflow: bool,
    pub halted: bool,
    pub carry: bool,
    pub div_by_zero: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
        not_zero: false,
        less_then: false,
        larger_then: false,
        equals: false,
        overflow: false,
        underflow: false,
        halted: false,
        carry: false,
        div_by_zero: false,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Nop,
    Hlt,
    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    Push,
    Pop,
    CompilerCall,
    Call,
    Ret,
    PushReg,
    PopReg,
    Store,
    Load,
    StoreRelative,
    LoadRelative,
    StackAdd,
    Deref,
    Lea,
    DerefAssign,
    DerefAssignRelative,
    Cmp,
    Jmp,
    Jz,
    Jnz,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    Equal,
    NotEqual,
}

impl From<u8> for Instruction {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Nop,
            1 => Self::Hlt,
            2 => Self::I32Add,
            3 => Self::I32Sub,
            4 => Self::I32Mul,
            5 => Self::I32Div,
            6 => Self::Push,
            7 => Self::Pop,
            8 => Self::CompilerCall,
            9 => Self::Call,
            10 => Self::Ret,
            11 => Self::PushReg,
            12 => Self::PopReg,
            13 => Self::Store,
            14 => Self::Load,
            15 => Self::StoreRelative,
            16 => Self::LoadRelative,
            17 => Self::StackAdd,
            18 => Self::Deref,
            19 => Self::Lea,
            20 => Self::DerefAssign,
            21 => Self::DerefAssignRelative,
            22 => Self::Cmp,
            23 => Self::Jmp,
            24 => Self::Jz,
            25 => Self::Jnz,
            26 => Self::Greater,
            27 => Self::GreaterEqual,
            28 => Self::Lesser,
            29 => Self::LesserEqual,
            30 => Self::Equal,
            31 => Self::NotEqual,
            _ => Self::Nop,
        }
    }
}

impl From<Instruction> for u8 {
    fn from(ins: Instruction) -> Self {
        unsafe {
            std::mem::transmute(ins)
        }
    }
}

struct Stack {
    stack: Vec<i32>,
    ptr: usize,
}

impl Stack {
    fn new(size: usize) -> Stack {
        Stack { stack: vec![0; size], ptr: size-1 }
    }

    fn get(&self, index: usize) -> i32 {
        self.stack[index]
    }

    fn set(&mut self, index: usize, val: i32) {
        self.stack[index] = val;
    }

    fn push(&mut self, val: i32) {
        if self.ptr >= self.stack.len() {
            panic!("Tried to push beyond stack limit! : {} / {}", self.ptr, self.ptr as u32 as i32);
        } else {
            self.stack[self.ptr] = val;
            self.ptr -= 1;
        }
    }

    fn pop(&mut self) -> i32 {
        self.ptr += 1;
        self.stack[self.ptr]
    }
}

fn debug(str: &str) {
    //print!("{}", str);
}

pub struct Instructions {
    instructions: Vec<u8>,
}

impl Instructions {
    fn push(&mut self, val: u8) {
        self.instructions.push(val);
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn push_instruction(&mut self, ins: Instruction) {
        self.instructions.push(u8::from(ins))
    }

    pub fn push_u8_operand(&mut self, val: u8) {
        self.instructions.push(val)
    }

    pub fn push_u16_operand(&mut self, val: u16) {
        self.push_u8_operand(val as u8);
        self.push_u8_operand((val >> 8) as u8);
    }

    pub fn push_u32_operand(&mut self, val: u32) {
        self.push_u16_operand(val as u16);
        self.push_u16_operand((val >> 16) as u16);
    }

    pub fn push_i32_operand(&mut self, val: i32) {
        self.push_u32_operand(bytemuck::cast(val))
    }

    pub fn set_u8_operand(&mut self, val: u8, index: usize) {
        self.instructions[index] = val;
    }

    pub fn set_u16_operand(&mut self, val: u16, index: usize) {
        self.set_u8_operand(val as u8, index);
        self.set_u8_operand((val >> 8u32) as u8, index+1);
    }

    pub fn set_u32_operand(&mut self, val: u32, index: usize) {
        self.set_u16_operand(val as u16, index);
        self.set_u16_operand((val >> 16u32) as u16, index+2);
    }

    pub fn set_i32_operand(&mut self, val: i32, index: usize) {
        self.set_u32_operand(bytemuck::cast(val), index);
    }

    pub fn get_u8(&self, index: usize) -> u8 {
        self.instructions[index]
    }

    pub fn get_u16(&self, index: usize) -> u16 {
        self.get_u8(index) as u16 + ((self.get_u8(index + 1) as u16) << 8)
    }

    pub fn get_u32(&self, index: usize) -> u32 {
        self.get_u16(index) as u32 + ((self.get_u16(index + 2) as u32) << 16)
    }

    pub fn get_i32(&self, index: usize) -> i32 {
        bytemuck::cast(self.get_u32(index))
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;
        while index < self.len() {
            let ins = Instruction::from(self.instructions[index]);
            write!(f, "{} : {:?}", index, ins)?;

            match ins {
                Instruction::StackAdd | Instruction::PushReg |
                Instruction::Call | Instruction::Push |
                Instruction::CompilerCall | Instruction::StoreRelative |
                Instruction::LoadRelative | Instruction::DerefAssign |
                Instruction::DerefAssignRelative | Instruction::Lea |
                Instruction::PopReg | Instruction::Store |
                Instruction::Load | Instruction::Jmp |
                Instruction::Jz | Instruction::Jnz => {
                    write!(f, " {}", self.get_i32(index+1))?;
                    index += 4;
                }
                _ => {}
            }

            writeln!(f, "")?;

            index += 1;
        }
        Ok(())
    }
}

impl Index<usize> for Instructions {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instructions[index]
    }
}

impl IndexMut<usize> for Instructions {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.instructions[index]
    }
}

pub struct Interpreter {
    stack: Stack,
    pub instructions: Instructions,
    ptr: usize,
    frame_ptr: usize,
    flags: Flags,
}

impl Interpreter {
    pub fn new(instructions: Vec<u8>) -> Interpreter {
        Interpreter {
            stack: Stack::new(1024),
            instructions: Instructions { instructions },
            ptr: 0,
            frame_ptr: 0,
            flags: Flags::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let ins = self.next_instruction();
            debug(&format!("{:?} ", ins));
            match ins {
                Instruction::Nop => {},
                Instruction::Hlt => return,
                Instruction::Lea => {
                    let location = self.next_i32() + self.frame_ptr as u32 as i32;
                    self.stack_push(location);
                    debug(&format!("{}\n", location));
                },
                Instruction::I32Add => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = a + b;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                },
                Instruction::I32Sub => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = a - b;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                },
                Instruction::I32Mul => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = a * b;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                },
                Instruction::I32Div => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = a / b;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                },
                Instruction::Push => {
                    let val = self.next_i32();
                    self.stack_push(val);
                    debug(&format!("{}\n", val));
                }
                Instruction::Pop => {
                    let val = self.stack_pop();
                    debug(&format!("{}\n", val));
                }
                Instruction::CompilerCall => {
                    let function = self.next_i32();
                    debug(&format!("{}\n", function));
                    match function {
                        0 => {},
                        1 => {
                            println!("Outputed: {}", self.stack.get(self.stack.ptr+1))
                        },
                        _ => panic!("Compiler call failed function with index does not exist : [{}]", function)
                    }
                }
                Instruction::Call => {
                    let destination = self.next_i32();
                    self.stack_push(self.ptr as u32 as i32);
                    self.stack_push(self.frame_ptr as u32 as i32);
                    self.ptr = destination as u32 as usize;
                    self.frame_ptr = self.stack.ptr;
                    debug(&format!("{}, {}\n", destination, self.frame_ptr));
                }
                Instruction::Ret => {
                    let frame_ptr = self.stack_pop();
                    self.frame_ptr = frame_ptr as u32 as usize;
                    let destination = self.stack_pop();
                    self.ptr = destination as u32 as usize;
                    debug(&format!("{}, {}\n", destination, self.frame_ptr));
                }
                Instruction::PopReg => {
                    let dst = self.next_u8();
                    let val = self.stack_pop();
                    debug(&format!("DST: {}, VAL: {}\n", dst, val));
                    match dst {
                        0 => {},
                        1 => self.ptr = val as u32 as usize,
                        2 => self.stack.ptr = val as u32 as usize,
                        3 => self.frame_ptr = val as u32 as usize,
                        _ => panic!(),
                    }
                }
                Instruction::PushReg => {
                    let src = self.next_u8();
                    debug(&format!("{}\n", src));
                    match src {
                        0 => {},
                        1 => self.stack_push(self.ptr as u32 as i32),
                        2 => self.stack_push(self.stack.ptr as u32 as i32),
                        3 => self.stack_push(self.frame_ptr as u32 as i32),
                        _ => panic!(),
                    }
                },
                Instruction::Load => {
                    let location = self.next_i32();
                    let val = self.stack.get(location as u32 as usize);
                    self.stack_push(val);
                    debug(&format!("&{}:${}\n", location, val));
                },
                Instruction::Store => {
                    let location = self.next_i32();
                    let val = self.stack_pop();
                    self.stack.set(location as u32 as usize, val);
                    debug(&format!("&{}:${}\n", location, val));
                }
                Instruction::LoadRelative => {
                    let location = self.frame_ptr as u32 as i32 + self.next_i32();
                    let val = self.stack.get(location as u32 as usize);
                    self.stack_push(val);
                    debug(&format!("&{}:${}\n", location - self.frame_ptr as u32 as i32, val));
                },
                Instruction::StoreRelative => {
                    let location = self.frame_ptr as u32 as i32 + self.next_i32();
                    let val = self.stack_pop();
                    self.stack.set(location as u32 as usize, val);
                    debug(&format!("&{}:${}\n", location - self.frame_ptr as u32 as i32, val));
                }
                Instruction::StackAdd => {
                    let offset = self.next_i32();
                    self.stack.ptr = (self.stack.ptr as u32 as i32 + offset) as u32 as usize;
                    debug(&format!("{}\n", offset));
                }
                Instruction::DerefAssignRelative => {
                    let ptr = self.frame_ptr as u32 as i32 + self.next_i32();
                    let location = self.stack.get(ptr as u32 as usize);
                    let val = self.stack_pop();
                    self.stack.set(location as u32 as usize, val);
                    debug(&format!("&{}:${}\n", location - self.frame_ptr as u32 as i32, val));
                }
                Instruction::DerefAssign => {
                    let ptr = self.next_i32();
                    let location = self.stack.get(ptr as u32 as usize);
                    let val = self.stack_pop();
                    self.stack.set(location as u32 as usize, val);
                    debug(&format!("&{}:${}\n", location, val));
                }
                Instruction::Deref => {
                    let ptr = self.stack_pop();
                    let val = self.stack.get(ptr as u32 as usize);
                    self.stack_push(val);
                    debug(&format!("&{}:${}\n", ptr, val));
                }
                Instruction::Cmp => {
                    let lhs = self.stack_pop();
                    let rhs = self.stack_pop();
                    let diff = lhs - rhs;
                    if diff < 0 {
                        self.flags.less_then = true;
                        self.flags.larger_then = false;
                        self.flags.not_zero = true;
                        self.flags.equals = false;
                    } else if diff > 0 {
                        self.flags.less_then = false;
                        self.flags.larger_then = true;
                        self.flags.not_zero = true;
                        self.flags.equals = false;
                    }
                    else {
                        self.flags.less_then = false;
                        self.flags.larger_then = false;
                        self.flags.not_zero = false;
                        self.flags.equals = true;
                    }
                },
                Instruction::Jmp => {
                    let dst = self.next_i32();
                    self.ptr = dst as u32 as usize;
                }
                Instruction::Jz => {
                    let dst = self.next_i32();
                    let val = self.stack_pop();
                    if val == 0 {
                        self.ptr = dst as u32 as usize;
                    }
                }
                Instruction::Greater => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = (a > b) as i32;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                }
                Instruction::GreaterEqual => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = (a >= b) as i32;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                }
                Instruction::Lesser => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = (a < b) as i32;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                }
                Instruction::LesserEqual => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = (a <= b) as i32;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                }
                Instruction::Equal => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = (a == b) as i32;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                }
                Instruction::NotEqual => {
                    let a = self.stack_pop();
                    let b = self.stack_pop();
                    let c = (a != b) as i32;
                    self.stack_push(c);
                    debug(&format!("{}, {}\n", a, b));
                }
                ins => panic!("Invalid instruction with op code of : {}", u8::from(ins)),
            }
        }
    }

    pub fn push_u8_operand(&mut self, val: u8) {
        self.instructions.push(val)
    }

    pub fn push_u16_operand(&mut self, val: u16) {
        self.push_u8_operand(val as u8);
        self.push_u8_operand((val >> 8) as u8);
    }

    pub fn push_u32_operand(&mut self, val: u32) {
        self.push_u16_operand(val as u16);
        self.push_u16_operand((val >> 16) as u16);
    }

    pub fn push_i32_operand(&mut self, val: i32) {
        self.push_u32_operand(bytemuck::cast(val))
    }

    pub fn set_u8_operand(&mut self, val: u8, index: usize) {
        self.instructions[index] = val;
    }

    pub fn set_u16_operand(&mut self, val: u16, index: usize) {
        self.set_u8_operand(val as u8, index);
        self.set_u8_operand((val >> 8u32) as u8, index+1);
    }

    pub fn set_u32_operand(&mut self, val: u32, index: usize) {
        self.set_u16_operand(val as u16, index);
        self.set_u16_operand((val >> 16u32) as u16, index+2);
    }

    pub fn set_i32_operand(&mut self, val: i32, index: usize) {
        //println!("i32_op:{}", val);
        self.set_u32_operand(bytemuck::cast(val), index);
    }

    fn get_u8(&self, index: usize) -> u8 {
        self.instructions[index]
    }

    fn get_u16(&self, index: usize) -> u16 {
        self.get_u8(index) as u16 + ((self.get_u8(index + 1) as u16) << 8)
    }

    fn get_u32(&self, index: usize) -> u32 {
        self.get_u16(index) as u32 + ((self.get_u16(index + 2) as u32) << 16)
    }

    fn next_instruction(&mut self) -> Instruction {
        if self.ptr >= self.instructions.len() {
            return Instruction::Hlt;
        }
        let ins = Instruction::from(self.get_u8(self.ptr));
        self.ptr += 1;
        ins
    }

    fn next_i32(&mut self) -> i32 {
        let val = bytemuck::cast(self.get_u32(self.ptr));
        self.ptr += 4;
        val
    }

    fn next_u8(&mut self) -> u8 {
        let val = self.get_u8(self.ptr);
        self.ptr += 1;
        val
    }

    fn stack_push(&mut self, val: i32) {
        //println!("Pushed: {}", val);
        self.stack.push(val);
    }

    fn stack_pop(&mut self) -> i32 {
        let val = self.stack.pop();
        //println!("Poped: {}", val);
        val
    }
}