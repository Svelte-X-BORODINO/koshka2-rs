// COMMIT FROM PROGRAMMING CLASS
use std::{process::exit, cell::Cell};
use crate::{video2::VideoController2, state::GetState};
pub const AX: usize = 0;
pub const BX: usize = 1;
pub const CX: usize = 2;
pub const DX: usize = 3;
pub const LP: usize = 4;
pub const CTL0: usize = 10; // CR0-like register
pub const CTL1: usize = 11; // CR3-like register
#[repr(align(64))]
#[derive(Clone)]
pub struct KoshkaCPU2 {
    pub k: [u16; 12], // 24 bytes
    pub kadv: Cell<u32>,
    pub memory: Box<[u8; 256*1024]>, // 8 bytes
    pub pc: u32, // 4 bytes
    pub sp: u32, // 4 bytes
    pub kflags: u8, // 1 byte
    pub current_page: u8, // 1 byte
    // size is 50 bytes, but 50 is not power of 2
    // so this struct is aligned to 64 bytes(64 bytes - CPU cash-line(thats good))
}

impl KoshkaCPU2 {
    pub fn new() -> Self {
        Self {
            k: [0; 12],
            kadv: Cell::new(0x00000000),
            memory: Box::new([0; 256*1024]),
            pc: 0x00002000,
            sp: 0xFFFE,
            kflags: 0b00000000,
            //*       CNZ--BI-
            
            current_page: 0,
        }
    }
    
    pub fn state(self) {
        GetState::state(&self);
    }

    pub fn panic_cpu(&self, res: &str) -> ! {
        VideoController2::disp(&format!("panic cpu#0 res={}", res).as_bytes());
        exit(1)
    }
    // someone: where is ALU functions?
    // me: Ziglang stole them
    
    #[inline]
    pub fn write8(&mut self, addr: u32, data: u8) {
        self.memory[addr as usize] = data;
    }

    #[inline]
    pub fn read8(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    pub fn push8(&mut self, data: u8) {
        self.sp -= 1;
        Self::write8(self, self.sp.try_into().unwrap(), data);
    }

    pub fn push16(&mut self, data: u16) {
        let low = (data << 8) as u8;
        let high = data as u8;
        Self::push8(self, low);
        Self::push8(self, high);
    }

    pub fn pop8(&mut self) -> u8 {
        self.sp += 1;
        self.memory[self.sp as usize]
    }

    pub fn pop16(&mut self) -> u16 {
        let low = self.pop8() as u16;
        let high = self.pop8() as u16;
        (high << 8) | low
    }

    pub fn show_stack(&self) {
        let start = (self.sp + 1) as usize;
        let end = self.sp as usize;
        
        if start > end {
            return;
        }

        println!("Depth   Value");
        println!("------  -------");

        for (i, addr) in (start..=end).rev().enumerate() {
            let value = self.memory[addr];
            println!("{:04X}     ${:04X}", i, value);
        }
    }

    pub fn show_mem(&self, start: usize, limit: usize) {
        if start >= 256 * 1024 {
            println!("Start address out of bounds");
            return;
        }

        let end = limit.min(256 * 1024);

        for i in start..end {
            let offset = i - start; 

            if offset % 16 == 0 {
                if offset > 0 {
                    println!(); 
                }
                print!("0x{:04X} | ", i);
            }

            print!("${:02X} ", self.memory[i]);
        }

        println!(); 
    }


}
