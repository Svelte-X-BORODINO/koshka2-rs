//! The Invoke Action Table
//! 
//! This shit is used to handle the interrupts
//! 
//! ---
//! 
//! ## Structs:
//! 
//! IATEntry:
//!     num unsig8:      number of interrupt
//!     handler unsig24:  address to handler of interrupt
//! 
//! IATable:
//!     entries [Option<IATEntry>; 256]: entries list
use crate::u24::u24;
#[derive(Clone, Copy)]
pub struct IATEntry {
    num: u8,
    handler: u24,
}

pub struct IATable {
    entries: [Option<IATEntry>; 256],
}

impl IATable {
    pub fn new() -> Self {
        Self {
            entries: [None; 256],
        }
    }

    pub fn insert_entry(&mut self, num: u8, handler: u24) {
        self.entries[num as usize] = Some(IATEntry { num, handler });
    }

    /// Load IAT from memory.
    ///
    /// Layout per entry (4 bytes):
    ///   [0] num (u8)
    ///   [1] handler low  (b1)
    ///   [2] handler mid  (b2)
    ///   [3] handler high (b3)
    ///
    /// If a slot looks empty (num==0 and handler==0), we store None.
    pub fn load_iat(&mut self, cpu: &mut crate::cpu2::KoshkaCPU2, base: u32) {
        for num in 0u8..=u8::MAX {
            let idx = num as usize;
            let pos = base.wrapping_add((num as u32) * 4);

            let read_num = cpu.read8(pos + 0);
            let b1: u24 = u24::from(cpu.read8(pos + 1));
            let b2: u24 = u24::from(cpu.read8(pos + 2));
            let b3: u24 = u24::from(cpu.read8(pos + 3));
            let handler: u24 = ((b3 << u24::from(16u8)) | (b2 << u24::from(8u8)) | b1) as u24;

            let is_empty = read_num == 0 && handler == u24::from(0u8);
            if is_empty {
                self.entries[idx] = None;
            } else {
                self.entries[idx] = Some(IATEntry {
                    num: read_num,
                    handler,
                });
            }
        }

        cpu.iatr = u24::from(base); 

    }
}