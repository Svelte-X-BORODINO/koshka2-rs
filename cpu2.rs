//! The Heart Of Koshka II RS CPU
//! 
//! This module is implements all CPU methods... WITHOUT STANDARD LIBRARY.
//! 
//! # Example:
//! ```no_run
//! let mut cpu = KoshkaCPU2::new(); // Creates a new CPU.
//! cpu.write8(0x12, 0x34); // Writes a byte 0x34 to offset 0x12.
//! let result = cpu.read8(0x12) // Reads a byte from offset 0x12 into variable 'result'.
//! cpu.state() // Prints the state of CPU.
//! ```

#![no_std]
use core::{cell::Cell, arch::asm};
use crate::{video2::VideoController2, state::GetState, u24::u24};
pub const AX: usize = 0;
pub const BX: usize = 1;
pub const CX: usize = 2;
pub const DX: usize = 3;
// COMMIT FROM PROGRAMMING CLASS
pub const LP: usize = 4;
pub const CTL0: usize = 10; // CR0-like register
pub const CTL1: usize = 11; // CR3-like register
#[repr(C, align(64))]
#[derive(Clone)]
/// # The CPU Structure.
pub struct KoshkaCPU2 {
    /// 12 Registers
    pub k: [u16; 12], // 24 bytes
    /// Source Index
    pub kadv: Cell<u32>, // 4 bytes
    /// Flash Memory
    pub memory: Box<[u8; 256*1024]>, // 8 bytes
    /// Program Counter 
    pub pc: u32, // 4 bytes
    /// Stack Pointer
    pub sp: u32, // 4 bytes
    /// Flags
    pub kflags: u8, // 1 byte
    /// Current Page
    pub current_page: u8, // 1 byte
    /// IAT Register
    pub iatr: crate::u24::u24, // 3 bytes
    // size is 53 bytes, but 53 is not power of 2
    // so this struct is aligned to 64 bytes(64 bytes - CPU cash-line(thats good))
}

fn exit(code: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,  
            in("rdi") code, 
            options(noreturn)
        )
    }
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
            iatr: u24::new(0x000000),
            current_page: 0,
        }
    }
    /// Get the state of CPU.
    pub fn state(&self) {
        GetState::state(self);
    }

    /// Panic with exit.
    pub fn panic_cpu(&self, res: &str) -> ! {
        VideoController2::disp(&format!("panic cpu#0 res={}", res).as_bytes());
        exit(1)
    }
    // someone: where is ALU functions?
    // me: Ziglang stole them
    
    /// Write a byte to memory.
    #[inline]
    pub fn write8(&mut self, addr: u32, data: u8) {
        self.memory[addr as usize] = data;
    }

    /// Read a byte from memory.
    #[inline]
    pub fn read8(&self, addr: u32) -> u8 {
        self.memory[addr as usize]
    }

    /// Push a byte into the stack.
    pub fn push8(&mut self, data: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.write8(self.sp, data);
    }

    /// Push a word into the stack.
    pub fn push16(&mut self, data: u16) {
        let low = data as u8;
        let high = (data >> 8) as u8;
        Self::push8(self, high);
        Self::push8(self, low);
    }

    /// Pop a byte from the stack.
    pub fn pop8(&mut self) -> u8 {
        let value = self.read8(self.sp);
        self.sp = self.sp.wrapping_add(1);
        value
    }

    /// Pop a word from the stack.
    pub fn pop16(&mut self) -> u16 {
        let low = self.pop8() as u16;
        let high = self.pop8() as u16;
        (high << 8) | low
    }

    /// Show the stack.
    pub fn show_stack(&self) {
        let mut sp = self.sp;
        let mut i = 0;
        
        println!("Depth   Address Value");
        println!("------  ------- -----");
        
        while self.memory[sp as usize] != 0 {
            let value = self.memory[sp as usize];
            println!("{:04}    ${:04X}   ${:02X}", i, sp, value);
            sp += 1;
            i += 1;
        }
    }

    /// Show memory from `start` to `limit`.
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
