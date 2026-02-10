use crate::cpu2::KoshkaCPU2;
use crate::isa::Instruction2;
pub struct KRSAssembler2;

impl KRSAssembler2 {
    pub fn exec(cpu: &mut KoshkaCPU2) {
        let opcode = cpu.memory[cpu.pc as usize];

        let inst_size = match opcode {
            // none               
            0x00 => {
                Instruction2::None();
            },
            // hlt                
            0x01 => {
                Instruction2::Hlt();
            },
            // stop               
            0x02 => {
                Instruction2::Stop();
            },
            // why div? Cuz i want
            // div ax, imm16      
            0x10 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Div(cpu, cpu.k[AX], arg);
            },
            // div ax, imm8       
            0x11 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Div(cpu, cpu.k[AX], arg);
            },
            // div bx imm16       
            0x12 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Div(cpu, cpu.k[BX], arg);
            },
            // div bx imm8        
            0x13 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Div(cpu, cpu.k[BX], arg);
            },
            // div cx imm16       
            0x14 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Div(cpu, cpu.k[CX], arg);
            },
            // div cx imm8        
            0x15 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Div(cpu, cpu.k[CX], arg);
            },
            // div dx imm16       
            0x16 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Div(cpu, cpu.k[DX], arg);
            },
            // div dx imm8        
            0x17 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Div(cpu, cpu.k[DX], arg);
            },
            // div reg reg        
            0x1A => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                Instruction2::DivRR(cpu, cpu.k[reg1], cpu.k[reg2]);
            },
            // sub ax imm16       
            0x20 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Sub(cpu, cpu.k[AX], arg);
            },
            // sub ax imm8        
            0x21 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Sub(cpu, cpu.k[AX], arg);
            },
            // sub bx imm16        
            0x22 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Sub(cpu, cpu.k[BX], arg);
            },
            // sub bx imm8        
            0x23 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Sub(cpu, cpu.k[BX], arg);
            },
            // sub cx imm16        
            0x24 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Sub(cpu, cpu.k[CX], arg);
            },
            // sub cx imm8
            0x25 => {
                let arg = cpu.memory[cpu.pc as usize + 1];
                Instruction2::Sub(cpu, cpu.k[CX], arg);
            }
            // sub dx imm16       
            0x26 => {
                let arg1 = cpu.read8(cpu.pc as usize + 1);
                let arg2 = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg1 << 8) | arg2; 
                Instruction2::Sub(cpu, cpu.k[DX], arg);
            },
            // sub dx imm8        
            0x27 => {
                let reg1 = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Sub(cpu, cpu.k[DX], arg);
            },
            // sub reg reg        
            0x2A => {
                let reg1 = cpu.read8(cpu.pc as usize + 1);
                let reg2 = cpu.read8(cpu.pc as usize + 2);
                Instruction2::SubRR(cpu, cpu.k[reg1], cpu.k[reg2]);
            },
            // cmp ax imm16       
            0x3C => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[AX], arg);
            },
            // cmp bx imm16
            0x3D => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[BX], arg);
            },
            // cmp cx imm16
            0x3E => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[CX], arg);
            },
            // cmp dx imm16
            0x3F => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Cmp(cpu, cpu.k[DX], arg);
            },
            // push imm16
            0x40 => {
                let arg_high = cpu.read8(cpu.pc as usize + 1);
                let arg_low = cpu.read8(cpu.pc as usize + 2);
                let arg = (arg_high << 8) | arg_low; 
                Instruction2::Push16(cpu, arg);
            },
            // push imm8
            0x41 => {
                let arg = cpu.read8(cpu.pc as usize + 1);
                Instruction2::Push8(cpu, arg);
            },
            // push reg 
            // example: push ax = 42 01; push ctl0 = 42 0A
            0x42 => {
                let reg_no = cpu.read8(cpu.pc as usize + 1);
                Instruction2::PushR(cpu, reg_no);
            },
            // pop reg
            0x43 => {
                let reg_no = cpu.read8(cpu.pc as usize + 1);
            }
            _ => {

            }
        };
    }
}