mod cpu2;
mod video2;
mod paging;
mod debug;
mod ia2;
mod asm;
mod state;

use crate::{cpu2::KoshkaCPU2, paging::Page};


fn main() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
    cpu.write8(0x80, 'H' as u8);
    cpu.write8(0x81, 'e' as u8);
    cpu.write8(0x82, 'l' as u8);
    cpu.write8(0x83, 'l' as u8);
    cpu.write8(0x84, 'o' as u8);
    cpu.write8(0x85, 0x00);
    cpu.kadv.set(0x80);
    while cpu.memory[cpu.kadv.get() as usize] != 0x00 {
        let mut ptr = cpu.kadv.get();
        print!("{}", cpu.memory[ptr as usize] as char);
        cpu.kadv.set(ptr.wrapping_add(1));    
    }
}
