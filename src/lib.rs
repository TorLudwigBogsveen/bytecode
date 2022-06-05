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

pub mod interpreter;

mod tests {
    use crate::interpreter::{Interpreter, Instruction, InstructionList};


    #[test]
    fn test_1() {
        let mut int = Interpreter::new(vec![
            u8::from(Instruction::Push), 10, 0, 0, 0,
            u8::from(Instruction::Push), 20, 0, 0, 0, 
            u8::from(Instruction::I32Add), 
            u8::from(Instruction::CompilerCall), 1, 0, 0, 0,
            u8::from(Instruction::Hlt)]);
        int.run();
    }

    #[test]
    fn test_2() {
        let mut instrs = InstructionList::new();
        instrs.push_instruction(Instruction::Push);
        instrs.push_i32_operand(10);
        instrs.push_instruction(Instruction::Push);
        instrs.push_i32_operand(20);
        instrs.push_instruction(Instruction::I32Add);
        instrs.push_instruction(Instruction::CompilerCall);
        instrs.push_i32_operand(1);
        instrs.push_instruction(Instruction::Hlt);

        let mut int = Interpreter::new(instrs.code);
        int.run();
    }
}