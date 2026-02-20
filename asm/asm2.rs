use crate::asm::isa2::Instruction2;
use crate::cpu2::{KoshkaCPU2, AX, BX, CX, DX};

pub struct KRSAssembler2;

#[allow(arithmetic_overflow)]
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
                1
            },
            // stop               
            // 0x02 => {
            //     Instruction2::Stop(cpu);
            //     1
            // },
            // div ax, imm16      
            0x10 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Div(cpu, AX as u16, imm as u16);
                3  
            },
            // div ax, imm8       
            0x11 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, AX as u16, imm as u16);
                2
            },
            // div bx imm16       
            0x12 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Div(cpu, BX as u16, imm as u16);
                3
            },
            // div bx imm8        
            0x13 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, BX as u16, imm as u16);
                2
            },
            // div cx imm16       
            0x14 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Div(cpu, CX as u16, imm as u16);
                3
            },
            // div cx imm8        
            0x15 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, CX as u16, imm as u16);
                2
            },
            // div dx imm16       
            0x16 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Div(cpu, DX as u16, imm as u16);
                3
            },
            // div dx imm8        
            0x17 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, DX as u16, imm as u16);
                2
            },
            // div reg reg        
            0x1A => {
                let reg1 = cpu.read8(cpu.pc + 1) as usize;
                let reg2 = cpu.read8(cpu.pc + 2) as usize;
                Instruction2::DivRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // sub ax imm16       
            0x20 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Sub(cpu, AX as u16, imm as u16);
                3
            },
            // sub ax imm8        
            0x21 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, AX as u16, imm as u16);
                2
            },
            // sub bx imm16        
            0x22 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Sub(cpu, BX as u16, imm as u16);
                3
            },
            // sub bx imm8        
            0x23 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, BX as u16, imm as u16);
                2
            },
            // sub cx imm16        
            0x24 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Sub(cpu, CX as u16, imm as u16);
                3
            },
            // sub cx imm8
            0x25 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, CX as u16, imm as u16);
                2
            },
            // sub dx imm16       
            0x26 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Sub(cpu, DX as u16, imm as u16);
                3
            },
            // sub dx imm8        
            0x27 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, DX as u16, imm as u16);
                2
            },
            // sub reg reg        
            0x2A => {
                let reg1 = cpu.read8(cpu.pc + 1) as usize;
                let reg2 = cpu.read8(cpu.pc + 2) as usize;
                Instruction2::SubRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // cmp ax imm16       
            0x3C => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Cmp(cpu, AX as u16, imm as u16);
                3
            },
            // cmp bx imm16
            0x3D => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Cmp(cpu, BX as u16, imm as u16);
                3
            },
            // cmp cx imm16
            0x3E => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Cmp(cpu, CX as u16, imm as u16);
                3
            },
            // cmp dx imm16
            0x3F => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Cmp(cpu, DX as u16, imm as u16);
                3
            },
            // push imm16
            0x40 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Push16(cpu, imm as u16);
                3
            },
            // push imm8
            0x41 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Push8(cpu, imm as u16);
                2
            },
            // push reg 
            0x42 => {
                let reg_no = cpu.read8(cpu.pc + 1);
                Instruction2::PushR(cpu, reg_no as u16); 
                2
            },
            // pop reg
            0x43 => {
                let reg_no = cpu.read8(cpu.pc + 1);
                Instruction2::Pop(cpu, reg_no as u16);
                2
            },
            // goto "somewhere"
            0xA0 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let addr = (high << 8) | low;
                Instruction2::Goto(cpu, addr as u32);
                3
            },
            0xA1 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let addr = (high << 8) | low;
                Instruction2::Gz(cpu, addr as u32);
                3
            },
            0xA2 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let addr = (high << 8) | low;
                Instruction2::Gnz(cpu, addr as u32);
                3
            },
            0xA3 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let addr = (high << 8) | low;
                Instruction2::Gc(cpu, addr as u32);
                3
            },
            0xA4 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let addr = (high << 8) | low;
                Instruction2::Gnc(cpu, addr as u32);  
                3
            },
            // mov ax $imm16
            0xB0 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Mov(cpu, AX as u16, imm as u16);
                3
            },
            0xB1 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Mov(cpu, BX as u16, imm as u16);
                3
            },
            0xB2 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Mov(cpu, CX as u16, imm as u16);
                3
            },
            0xB3 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high << 8) | low;
                Instruction2::Mov(cpu, DX as u16, imm as u16);
                3
            },
            0xBA => {
                // TODO: inc 
                3
            }
            _ => {
                cpu.panic_cpu(&format!("Unknown opcode: 0x{:02X}", opcode));
            }
        };

        cpu.pc += inst_size;
    }
}