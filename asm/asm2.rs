use crate::asm::isa2::Instruction2;
use crate::cpu2::{KoshkaCPU2, AX, BX, CX, DX};
use crate::video2::VideoKontroller2;
pub struct KRSAssembler2;



impl KRSAssembler2 {
    pub fn exec(cpu: &mut KoshkaCPU2, vc: &mut VideoKontroller2) {
        let opcode = cpu.memory[cpu.pc as usize];

        let inst_size = match opcode {
            // none
            0x00 => {
                Instruction2::None(cpu);
                1
            },
            0x01 => {
                Instruction2::Hlt();
                0
            },
            // div ax, imm16      
            0x03 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Div(cpu, AX as u16, imm);
                3  
            },
            // div ax, imm8       
            0x04 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, AX as u16, imm as u16);
                2
            },
            // div bx imm16       
            0x05 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Div(cpu, BX as u16, imm);
                3
            },
            // div bx imm8        
            0x06 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, BX as u16, imm as u16);
                2
            },
            // div cx imm16       
            0x07 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Div(cpu, CX as u16, imm);
                3
            },
            // div cx imm8        
            0x08 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, CX as u16, imm as u16);
                2
            },
            // div dx imm16       
            0x09 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Div(cpu, DX as u16, imm);
                3
            },
            // div dx imm8        
            0x0A => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Div(cpu, DX as u16, imm as u16);
                2
            },
            // div reg reg        
            0x0B => {
                let reg1 = cpu.read8(cpu.pc + 1) as usize;
                let reg2 = cpu.read8(cpu.pc + 2) as usize;
                Instruction2::DivRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // sub ax imm16       
            0x0C => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Sub(cpu, AX as u16, imm);
                3
            },
            // sub ax imm8        
            0x0D => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, AX as u16, imm as u16);
                2
            },
            // sub bx imm16        
            0x0E => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Sub(cpu, BX as u16, imm);
                3
            },
            // sub bx imm8        
            0x0F => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, BX as u16, imm as u16);
                2
            },
            0x10 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Sub(cpu, CX as u16, imm);
                3
            },
            // sub cx imm8
            0x11 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, CX as u16, imm as u16);
                2
            },
            // sub dx imm16       
            0x12 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Sub(cpu, DX as u16, imm);
                3
            },
            // sub dx imm8        
            0x13 => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Sub(cpu, DX as u16, imm as u16);
                2
            },
            // sub reg reg        
            0x14 => {
                let reg1 = cpu.read8(cpu.pc + 1) as usize;
                let reg2 = cpu.read8(cpu.pc + 2) as usize;
                Instruction2::SubRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // add ax $imm16
            0x15 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Add(cpu, AX as u16, imm as u16);
                3
            },
            // add bx $imm16
            0x16 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Add(cpu, BX as u16, imm as u16);
                3
            },
            // add cx $imm16
            0x17 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Add(cpu, CX as u16, imm as u16);
                3
            },
            // add dx $imm16
            0x18 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Add(cpu, DX as u16, imm as u16);
                3
            },
            // add reg reg
            0x19 => {
                let reg1 = cpu.read8(cpu.pc + 1) as usize;
                let reg2 = cpu.read8(cpu.pc + 2) as usize;
                Instruction2::AddRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // cmp ax $imm16       
            0x1A => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Cmp(cpu, AX as u16, imm);
                3
            },
            // cmp bx $imm16
            0x1B => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Cmp(cpu, BX as u16, imm);
                3
            },
            // cmp cx $imm16
            0x1C => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Cmp(cpu, CX as u16, imm);
                3
            },
            // cmp dx $imm16
            0x1D => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Cmp(cpu, DX as u16, imm);
                3
            },
            // push $imm16
            0x1E => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Push16(cpu, imm);
                3
            },
            // push $imm8
            0x1F => {
                let imm = cpu.read8(cpu.pc + 1);
                Instruction2::Push8(cpu, imm as u16);
                2
            },
            // push reg 
            0x20 => {
                let reg_no = cpu.read8(cpu.pc + 1);
                Instruction2::PushR(cpu, reg_no as u16); 
                2
            },
            // pop reg
            0x21 => {
                let reg_no = cpu.read8(cpu.pc + 1);
                Instruction2::Pop(cpu, reg_no as u16);
                2
            },
            // goto "somewhere"
            0x22 => {
                let low = cpu.read8(cpu.pc + 1) as u16;
                let high = cpu.read8(cpu.pc + 2) as u16;
                let addr = high.wrapping_shl(8).wrapping_add(low);
                Instruction2::Goto(cpu, addr as u32);
                3
            },
            // gz "somewhere"
            0x23 => {
                let low = cpu.read8(cpu.pc + 1) as u16;
                let high = cpu.read8(cpu.pc + 2) as u16;
                let addr = high.wrapping_shl(8).wrapping_add(low);
                Instruction2::Gz(cpu, addr as u32);
                3
            },
            // gnz "somewhere"
            0x24 => {
                let low = cpu.read8(cpu.pc + 1) as u16;
                let high = cpu.read8(cpu.pc + 2) as u16;
                let addr = high.wrapping_shl(8).wrapping_add(low);
                Instruction2::Gnz(cpu, addr as u32);
                3
            },
            // gc "somewhere"
            0x25 => {
                let low = cpu.read8(cpu.pc + 1) as u16;
                let high = cpu.read8(cpu.pc + 2) as u16;
                let addr = high.wrapping_shl(8).wrapping_add(low);
                Instruction2::Gc(cpu, addr as u32);
                3
            },
            // gnc "somewhere"
            0x26 => {
                let low = cpu.read8(cpu.pc + 1) as u16;
                let high = cpu.read8(cpu.pc + 2) as u16;
                let addr = high.wrapping_shl(8).wrapping_add(low);
                Instruction2::Gnc(cpu, addr as u32);  
                3
            },
            // mul ax $imm16
            0x27 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mul(cpu, AX as u16, imm as u16);
                3
            },
            // mul bx $imm16
            0x28 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mul(cpu, BX as u16, imm as u16);
                3
            },
            // mul cx $imm16
            0x29 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mul(cpu, CX as u16, imm as u16);
                3
            },
            // mul dx $imm16
            0x2A => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mul(cpu, DX as u16, imm as u16);
                3
            },
            // mul reg reg
            0x2B => {
                let reg1 = cpu.read8(cpu.pc + 1) as usize;
                let reg2 = cpu.read8(cpu.pc + 2) as usize;
                Instruction2::MulRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // mov ax $imm16
            0x2C => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mov(cpu, AX as u16, imm);
                3
            },
            // mov bx $imm16
            0x2D => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mov(cpu, BX as u16, imm);
                3
            },
            // mov cx $imm16
            0x2E => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mov(cpu, CX as u16, imm);
                3
            },
            // mov dx $imm16
            0x2F => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Mov(cpu, DX as u16, imm);
                3
            },
            // mov adv $imm24
            0x30 => {
                let b1 = cpu.read8(cpu.pc + 1) as u32;  
                let b2 = cpu.read8(cpu.pc + 2) as u32;
                let b3 = cpu.read8(cpu.pc + 3) as u32;              
                let imm24 = (b3 << 16) | (b2 << 8) | b1;
                VideoKontroller2::dispd(vc, &format!("kadv.set({})\n", imm24));
                cpu.kadv.set(imm24);
                VideoKontroller2::dispd(vc, &format!("kadv: {:06X}\n", cpu.kadv.get()));
                4
            }
            // inc ax
            0x31 => {
                Instruction2::IncR(cpu, AX);
                1
            },
            // inc bx
            0x32 => {
                Instruction2::IncR(cpu, BX);
                1
            },
            // inc cx
            0x33 => {
                Instruction2::IncR(cpu, CX);
                1
            },
            // inc dx
            0x34 => {
                Instruction2::IncR(cpu, DX);
                1
            },
            // xor ax $imm16
            0x35 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Xor(cpu, AX as u16, imm);
                3
            },
            // and ax $imm16
            0x36 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::And(cpu, AX as u16, imm);
                3
            },
            // and bx $imm16
            0x37 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::And(cpu, BX as u16, imm);
                3
            },
            // and cx $imm16
            0x38 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::And(cpu, CX as u16, imm);
                3
            },
            // and dx $imm16
            0x39 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::And(cpu, DX as u16, imm);
                3
            },
            // and reg reg
            0x3A => {
                let reg1 = cpu.read8(cpu.pc + 1);
                let reg2 = cpu.read8(cpu.pc + 2);
                Instruction2::AndRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // or ax $imm16
            0x3B => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Or(cpu, AX as u16, imm);
                3
            },
            // or bx $imm16
            0x3C => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Or(cpu, BX as u16, imm);
                3
            },
            // or cx $imm16
            0x3D => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Or(cpu, CX as u16, imm);
                3
            },
            // or dx $imm16
            0x3E => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Or(cpu, DX as u16, imm);
                3
            },
            // or reg reg
            0x3F => {
                let reg1 = cpu.read8(cpu.pc + 1);
                let reg2 = cpu.read8(cpu.pc + 2);
                Instruction2::OrRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            // not ax
            0x40 => {
                Instruction2::Not(cpu, 0);
                3
            }
            // not bx
            0x41 => {
                Instruction2::Not(cpu, 1);
                3
            },
            // not bx
            0x42 => {
                Instruction2::Not(cpu, 2);
                3
            },
            // not cx
            0x43 => {
                Instruction2::Not(cpu, 3);
                3
            },
            // not dx
            0x44 => {
                Instruction2::Not(cpu, 4);
                3
            },
            // ...
            // xor bx $imm16
            0xC1 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Xor(cpu, BX as u16, imm);
                3
            },
            // xor cx $imm16 (maybe unnesecary)
            0xC2 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Xor(cpu, CX as u16, imm);
                3
            },
            // xor dx $imm16
            0xC3 => {
                let low = cpu.read8(cpu.pc + 1);
                let high = cpu.read8(cpu.pc + 2);
                let imm = (high as u16).wrapping_shl(8).wrapping_add(low as u16);
                Instruction2::Xor(cpu, DX as u16, imm);
                3
            },
            // xor reg reg
            0xCC => { // OMG INT3 REFERENCE!?!?!?!?!!!???!?!?!?!!?1/1/1/??!??!?!?!?!?1872163872163721637821638721?!?!?!
                let reg1 = cpu.read8(cpu.pc + 1);
                let reg2 = cpu.read8(cpu.pc + 2);
                Instruction2::XorRR(cpu, reg1 as u16, reg2 as u16);
                3
            },
            _ => {
                cpu.panic_cpu(&format!("illegal_inst_${:02x}", opcode));
            }
        };

        cpu.pc = cpu.pc.wrapping_add(inst_size);
    }
}