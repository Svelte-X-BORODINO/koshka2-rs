use crate::cpu::KoshkaCPU2;
use crate::debug::KoshkaDB;
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
        Common(cpu, 1);
    }

    pub fn Mov(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = value;
        Common(cpu, 2);
    }

    pub fn MovRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg2 as usize];
        Common(cpu, 2);
    }

    pub fn Add(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_add(value);
        Common(cpu, 2);
    }

    pub fn AddRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_add(cpu.k[reg2 as usize]);
        Common(cpu, 2);

    }

    pub fn Sub(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_sub(value);
        Common(cpu, 2);
    }

    pub fn SubRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_sub(cpu.k[reg2 as usize]);
        Common(cpu, 2);
    }

    pub fn Mul(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_mul(value);
        Common(cpu, 2);
    }

    pub fn MulRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_mul(cpu.k[reg2 as usize]);
        Common(cpu, 2);
    }

    pub fn Div(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        if value == 0 {
            panic_cpu("div_by_zero");
        }
        cpu.k[reg as usize] = cpu.k[reg as usize].wrapping_div(value);
        Common(cpu, 2);
    }

    pub fn DivRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        if cpu.k[reg2] == 0 {
            panic!("div_by_zero");
        }
        cpu.k[reg1 as usize] = cpu.k[reg1 as usize].wrapping_div(cpu.k[reg2 as usize]);
        Common(cpu, 2);
    }

    pub fn Ldb(cpu: &mut KoshkaCPU2, dest: u32, data: u8) {
        cpu.memory[dest as usize] = data;
    }

    pub fn Ldw(cpu: &mut KoshkaCPU2, dest: u32, data: u16) {
        cpu.memory[dest as usize] = data as u8;
        cpu.memory[dest + 1 as usize] = data >> 8 as u8;
    }

    pub fn Push(cpu: &mut KoshkaCPU2, value: u16) {
        cpu.push(value);
        Common(cpu, 2);
    }

    pub fn PushR(cpu: &mut KoshkaCPU2, reg: u16) {
        cpu.push16(cpu.k[reg as usize]);
        Common(cpu, 2);
    }

    pub fn Pop(cpu: &mut KoshkaCPU2, reg: u16) {
        cpu.k[reg as usize] = cpu.pop();
        Common(cpu, 2);

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
            Common(cpu, 4);
        }
    }
    
    pub fn Gc(cpu: &mut KoshkaCPU2, addr: u32) {
        if CF(cpu) {
            Self::Goto(cpu, addr);
        } else {
            Common(cpu, 4);
        }
    }

    pub fn Gnc(cpu: &mut KoshkaCPU2, addr: u32) {
        if !CF(cpu) {
            Self::Goto(cpu, addr);
        } else {
            Common(cpu, 4);
        }
    }

    pub fn Gsub(cpu: &mut KoshkaCPU2, addr: u32) { // go to sub (same as x86-`call`)
        cpu.push(cpu.pc);
        cpu.pc = addr;
    }

    pub fn Done(cpu: &mut KoshkaCPU2) {
        cpu.pc = cpu.pop();
    }

    pub fn Cmp(cpu: &mut KoshkaCPU2, reg: u16, value: u16) {
        if cpu.k[reg] != value {
            SET_ZF(cpu);
        } else {
            Common(3);
        }
    }

    pub fn CmpRR(cpu: &mut KoshkaCPU2, reg1: u16, reg2: u16) {
        if cpu.k[reg1] != cpu.k[reg2] {
            SET_ZF(cpu);
        } else {
            Common(3);
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
    pub fn Stop(cpu: &mut KoshkaCPU2) {
        VideoController2::dispd(&format!("Trapped at PC:{}", cpu.pc));
        KoshkaDB::shell();
    }

    pub fn Hlt() -> ! {
        loop {}
    }
}
