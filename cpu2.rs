use std::process::exit;
use std::fmt::{self, format};
use crate::{video2::VideoController2, paging::Page, ia2::InvokeAction2};
pub const AX: usize = 0;
pub const BX: usize = 1;
pub const CX: usize = 2;
pub const DX: usize = 3;
pub const CTL0: usize = 11; // почти CRO
pub const CTL1: usize = 12; // почти CR4
pub struct KoshkaCPU2 {
    pub k: [u16; 12],
    pub kadv: *mut u32, 
    pub memory: [u8; 256*1024],
    pub pc: u32,
    pub sp: u32,
    pub kflags: u8,
    pub current_page: u8,
}

impl KoshkaCPU2 {
    pub fn new() -> Self {
        Self {
            k: [0; 12],
            kadv: std::ptr::null::<()>() as *mut u32,
            memory: [0; 256*1024],
            pc: 0,
            sp: 0xFFFE,
            kflags: 0b00000000,
            //*       CNZ--BI-
            
            current_page: 0,
        }
    }
    
    pub fn panic_cpu(&self, res: &str) -> ! {
        VideoController2::disp(&format!("panic cpu#0 res={}", res).as_bytes());
        exit(1)
    }
    // someone: where is ALU functions?
    // me: Ziglang stole them
    
    pub fn write(&mut self, addr: u32, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn push8(&mut self, data: u8) {
        Self::write(self, self.sp.try_into().unwrap(), data);
        self.sp -= 1;
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

    pub fn clear_mem(&mut self, limit: u32) {
        self.memory[0..limit as usize].fill(0);
    }
}