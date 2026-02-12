use crate::asm::isa2::Instruction2;
use crate::cpu2::{KoshkaCPU2, AX, BX, CX, DX};
use crate::isa::Instruction2;
pub struct KRSAssembler2;

impl KRSAssembler2 {
    pub fn exec(cpu: &mut KoshkaCPU2) {
        let opcode = cpu.memory[cpu.pc as usize];

        let inst_size = match opcode {
            // none               
            0x00 => {
                Instruction2::None(cpu);
                1
            },
            // hlt                
            0x01 => {
                Instruction2::Hlt();
            },
            // stop               
            0x02 => {
                Instruction2::Stop(cpu);
                1
            },
            // why div? Cuz i want
            // div ax, imm16      
            0x10 => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Div(cpu, cpu.k[AX], arg as u16);
                4
            },
            // div ax, imm8       
            0x11 => {
                let arg = cpu.read8(cpu.pc);
                Instruction2::Div(cpu, cpu.k[AX], arg as u16);
                3
            },
            // div bx imm16       
            0x12 => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Div(cpu, cpu.k[BX], arg);
                4
            },
            // div bx imm8        
            0x13 => {
                let arg = cpu.read8(cpu.pc);
                Instruction2::Div(cpu, cpu.k[BX], arg as u16);
                3
            },
            // div cx imm16       
            0x14 => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Div(cpu, cpu.k[CX], arg);
                4
            },
            // div cx imm8        
            0x15 => {
                let arg = cpu.read8(cpu.pc);
                Instruction2::Div(cpu, cpu.k[CX], arg as u16);
                3
            },
            // div dx imm16       
            0x16 => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Div(cpu, cpu.k[DX], arg);
                4
            },
            // div dx imm8        
            0x17 => {
                let arg = cpu.read8(cpu.pc);
                Instruction2::Div(cpu, cpu.k[DX], arg as u16);
                3
            },
            // div reg reg        
            0x1A => {
                let arg_high = cpu.read8(cpu.pc + 1);
                let arg_low = cpu.read8(cpu.pc + 2);
                Instruction2::DivRR(cpu, cpu.k[reg1 as usize], cpu.k[reg2 as usize]);
                3
            },
            // sub ax imm16       
            0x20 => {
                let arg_high = cpu.read8(cpu.pc + 1);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Sub(cpu, cpu.k[AX], arg as u16);
                4
            },
            // sub ax imm8        
            0x21 => {
                let arg = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, cpu.k[AX], arg as u16);
                3
            },
            // sub bx imm16        
            0x22 => {
                let arg_high = cpu.read8(cpu.pc + 1);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Sub(cpu, cpu.k[BX], arg as u16);
                4
            },
            // sub bx imm8        
            0x23 => {
                let arg = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, cpu.k[BX], arg as u16);
                3
            },
            // sub cx imm16        
            0x24 => {
                let arg_high = cpu.read8(cpu.pc + 1);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Sub(cpu, cpu.k[CX], arg as u16);
                4
            },
            // sub cx imm8
            0x25 => {
                let arg = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, cpu.k[CX], arg as u16);
                3
            }
            // sub dx imm16       
            0x26 => {
                let arg1 = cpu.read8(cpu.pc + 1);
                let arg2 = cpu.read8(cpu.pc + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Sub(cpu, cpu.k[DX], arg as u16);
                4
            },
            // sub dx imm8        
            0x27 => {
                let arg = cpu.read8(cpu.pc);
                Instruction2::Sub(cpu, cpu.k[DX], arg as u16);
                3
            },
            // sub reg reg        
            0x2A => {
                let reg1 = cpu.read8(cpu.pc);
                let reg2 = cpu.read8(cpu.pc + 2);
                Instruction2::SubRR(cpu, cpu.k[reg1 as usize], cpu.k[reg2 as usize]);
                3
            },
            // cmp ax imm16       
            0x3C => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[AX], arg as u16);
                4
            },
            // cmp bx imm16
            0x3D => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[BX], arg as u16);
                4
            },
            // cmp cx imm16
            0x3E => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[CX], arg as u16);
                4
            },
            // cmp dx imm16
            0x3F => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[DX], arg as u16);
                4
            },
            // push imm16
            0x40 => {
                let arg_high = cpu.read8(cpu.pc);
                let arg_low = cpu.read8(cpu.pc + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Push16(cpu, arg.into());
                3
            },
            // push imm8
            0x41 => {
                let arg = cpu.read8(cpu.pc);
                Instruction2::Push8(cpu, arg as u16);
                2
            },
            // push reg 
            // example: push ax = 42 01; push ctl0 = 42 0A
            0x42 => {
                let reg_no = cpu.read8(cpu.pc);
                Instruction2::PushR(cpu, reg_no as u16);
                2
            },
            // pop reg
            0x43 => {
                let reg_no = cpu.read8(cpu.pc);
                Instruction2::Pop(cpu, reg_no as u16);
                2
            },
            // goto "somewhere"
            0xA0 => {
                4
            },
            _ => {
                0
            }
        };

        cpu.pc += inst_size;
    }
}
