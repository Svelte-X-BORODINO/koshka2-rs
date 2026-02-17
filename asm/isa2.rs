use crate::cpu2::KoshkaCPU2;
// use crate::debug::KoshkaDB;
use crate::cpu2::{AX, BX, CX, DX};
use crate::video2::VideoController2;
fn Common(cpu: &mut KoshkaCPU2, n: u32) -> u32 {
    cpu.pc = cpu.pc.wrapping_add(n);
    n
}

fn CF(cpu: &mut KoshkaCPU2) -> bool { if cpu.kflags & 128 != 0 {true} else {false} }
fn NF(cpu: &mut KoshkaCPU2) -> bool { if cpu.kflags & 64 != 0 {true} else {false} }
fn ZF(cpu: &mut KoshkaCPU2) -> bool { if cpu.kflags & 32 != 0 {true} else {false} }
fn BF(cpu: &mut KoshkaCPU2) -> bool { if cpu.kflags & 4 != 0 {true} else {false} }
fn IF(cpu: &mut KoshkaCPU2) -> bool { if cpu.kflags & 2 != 0 {true} else {false} }

fn SET_CF(cpu: &mut KoshkaCPU2) { cpu.kflags |= 128 }
fn SET_NF(cpu: &mut KoshkaCPU2) { cpu.kflags |= 64 }
fn SET_ZF(cpu: &mut KoshkaCPU2) { cpu.kflags |= 32 }
fn SET_BF(cpu: &mut KoshkaCPU2) { cpu.kflags |= 4 }
fn SET_IF(cpu: &mut KoshkaCPU2) { cpu.kflags |= 2 }

fn CLEAR_CF(cpu: &mut KoshkaCPU2) { cpu.kflags &= !128  }
fn CLEAR_NF(cpu: &mut KoshkaCPU2) { cpu.kflags &= !64  }
fn CLEAR_ZF(cpu: &mut KoshkaCPU2) { cpu.kflags &= !32  }
fn CLEAR_BF(cpu: &mut KoshkaCPU2) { cpu.kflags &= !4  }
fn CLEAR_IF(cpu: &mut KoshkaCPU2) { cpu.kflags &= !2  }

pub struct Instruction2;

impl Instruction2 {
    pub fn None(cpu: &mut KoshkaCPU2) {
    }

    pub fn Mov(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = value;
    }

    pub fn MovRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg2 as usize];
    }

    pub fn Add(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_add(value);
    }

    pub fn AddRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_add(cpu.k[reg2 as usize]);

    }

    pub fn Sub(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_sub(value);
    }

    pub fn SubRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_sub(cpu.k[reg2 as usize]);
    }

    pub fn Mul(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_mul(value);
    }

    pub fn MulRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_mul(cpu.k[reg2 as usize]);
    }

    pub fn Div(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        if value == 0 {
            cpu.panic_cpu("div_by_zero");
        }
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_div(value);
    }

    pub fn DivRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        if cpu.k[reg2 as usize] == 0 {
            cpu.panic_cpu("div_by_zero");
        }
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_div(cpu.k[reg2 as usize]);
    }

    pub fn Ldb(cpu: &mut KoshkaCPU2, dest: u32, data: u8) {
        cpu.memory[dest as usize] = data;
    }

    pub fn Ldw(cpu: &mut KoshkaCPU2, dest: u32, data: u16) {
        cpu.memory[dest as usize] = data as u8;
        cpu.memory[dest.wrapping_add(1) as usize] = (data >> 8 as u8) as u8;
    }

    pub fn Push8(cpu: &mut KoshkaCPU2, value: u16) {
        cpu.push8(value as u8);
    }

    pub fn Push16(cpu: &mut KoshkaCPU2, value: u16) {
        cpu.push16(value as u16);
    }

    pub fn PushR(cpu: &mut KoshkaCPU2, reg: u16) {
        cpu.push16(cpu.k[reg as usize]);
    }

    pub fn Pop(cpu: &mut KoshkaCPU2, reg: u16) {
        cpu.k[reg as usize] = cpu.pop8().into();

    }

    pub fn Goto(cpu: &mut KoshkaCPU2, addr: u32) {
        cpu.pc = addr;
    }

    pub fn Gz(cpu: &mut KoshkaCPU2, addr: u32) {
        if ZF(cpu) {
            Self::Goto(cpu, addr);
        } else {
            Common(cpu, 4);
        }
    }

    pub fn Gnz(cpu: &mut KoshkaCPU2, addr: u32) {
        if !ZF(cpu) {
            Self::Goto(cpu, addr);
        } else {
        }
    }
    
    pub fn Gc(cpu: &mut KoshkaCPU2, addr: u32) {
        if CF(cpu) {
            Self::Goto(cpu, addr);
        } else {
        }
    }

    pub fn Gnc(cpu: &mut KoshkaCPU2, addr: u32) {
        if !CF(cpu) {
            Self::Goto(cpu, addr);
        } else {
        }
    }

    pub fn Gsub(cpu: &mut KoshkaCPU2, addr: u32) { // go to sub (same as x86-`call`)
        cpu.push8(cpu.pc as u8);
        cpu.pc = addr;
    }

    pub fn Done(cpu: &mut KoshkaCPU2) {
        cpu.pc = cpu.pop8().into();
    }

    pub fn Cmp(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        if cpu.k[reg as usize] != value {
            SET_ZF(cpu);
        } else {
            ()
        }
    }

    pub fn CmpRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        if cpu.k[reg1 as usize] != cpu.k[reg2 as usize] {
            SET_ZF(cpu);
        } else {
            ()
        }
    }


    pub fn And(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize] & value;
        Common(cpu, 2);
    }

    pub fn AndRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize] & cpu.k[reg2 as usize];
        Common(cpu, 2);
    }

    pub fn Or(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize] | value;
        Common(cpu, 2);
    }

    pub fn OrRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize] | cpu.k[reg2 as usize];
        Common(cpu, 2);
    }

    pub fn Xor(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize] ^ value;
        Common(cpu, 2);
    }

    pub fn XorRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize] ^ cpu.k[reg2 as usize];
        Common(cpu, 2);
    }
    
    // * happy new 2026!
    //pub fn Stop(cpu: &mut KoshkaCPU2) {
    //    VideoController2::dispd(&format!("Trapped at PC:{}", cpu.pc).as_bytes());
    //    KoshkaDB::shell();
    //}

    pub fn Hlt() -> ! {
        loop {}
    }
}
