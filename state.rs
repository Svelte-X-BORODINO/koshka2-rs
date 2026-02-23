use std::{fmt::UpperHex, cell::Cell};

use crate::cpu2::{KoshkaCPU2, AX, BX, CX, DX, CTL0, CTL1};

macro_rules! get_reg {
    () => {
        compile_error!("Invalid get_reg!")
    };
    ($cpu:expr, AX) => {
        print!("AX=${:04X}  ", $cpu.k[AX])
    };
    ($cpu:expr, BX) => {
        print!("BX=${:04X}  ", $cpu.k[BX])
    };
    ($cpu:expr, CX) => {
        print!("CX=${:04X}  ", $cpu.k[CX])
    };
    ($cpu:expr, DX) => {
        print!("DX=${:04X}  ", $cpu.k[DX])
    };
    ($cpu:expr, CTL0) => {
        print!("CTL0=${:X}  ", $cpu.k[CTL0])
    };
    ($cpu:expr, CTL1) => {
        print!("CTL1=${:X}  ", $cpu.k[CTL1])
    };
}
pub trait GetState {
    fn state(&self);
}

impl GetState for KoshkaCPU2 {
    fn state(&self) {
        let kadv = self.kadv.get();
        get_reg!(self, AX);
        get_reg!(self, BX);
        println!();
        get_reg!(self, CX);
        get_reg!(self, DX);
        println!("ADV=${:04X}  ", kadv);
        get_reg!(self, CTL0);
        get_reg!(self, CTL1);
    }
}