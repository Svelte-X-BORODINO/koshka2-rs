mod cpu2;
mod video2;
mod paging;
mod debug;
mod ia2;
mod asm;
mod state;
mod fpu;
mod u24;
use crate::{asm::asm2::KRSAssembler2, cpu2::KoshkaCPU2, paging::Page, video2::VideoKontroller2};

use ia2::iat::IATable;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[doc = "Test of the paging I/O work ability"]
    fn test_page_read8() {
        let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
        Page::set_page(&mut cpu, 2);
        Page::page_write8(&mut cpu, 0x0100, 0xB0);
        Page::page_write8(&mut cpu, 0x0101, 0x42);
        Page::page_write8(&mut cpu, 0x0102, 0x00);
        Page::page_write8(&mut cpu, 0x0103, 0xB9);
        assert_eq!(Page::page_read8(&mut cpu, 0x0100), 0xB0);
        assert_eq!(Page::page_read8(&mut cpu, 0x0101), 0x42);
        assert_eq!(Page::page_read8(&mut cpu, 0x0102), 0x00);
        assert_eq!(Page::page_read8(&mut cpu, 0x0103), 0xB9);
    }

    #[test]
    #[doc = "Test of the paging I/O work ability, but its write"]

    fn test_page_write8() {
        let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
        Page::set_page(&mut cpu, 2);
        Page::page_write8(&mut cpu, 0x0000, 0xB0);
        Page::page_write8(&mut cpu, 0x0001, 0x42);
        Page::page_write8(&mut cpu, 0x0002, 0x00);
        Page::page_write8(&mut cpu, 0x0003, 0xB9);
        assert_eq!(Page::page_read8(&mut cpu, 0x0000), 0xB0);
        assert_eq!(Page::page_read8(&mut cpu, 0x0001), 0x42);
        assert_eq!(Page::page_read8(&mut cpu, 0x0002), 0x00);
        assert_eq!(Page::page_read8(&mut cpu, 0x0003), 0xB9);
    }

    #[test]
    #[doc = "Test of the stack work ability"]
    fn test_push8() {
        let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
        cpu.push8(0xB9); 
        cpu.push8(0x00); 
        assert_eq!(cpu.pop8(), 0x00);
        assert_eq!(cpu.pop8(), 0xB9);
    }

    #[test]
    #[doc = "Test of the Invoke Action Table work ability"]

    fn test_load_iat() {
        let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
        let mut iat: IATable = IATable::new();
        Page::set_page(&mut cpu, 2);
        Page::page_write8(&mut cpu, 0x0000, 0xB0);
        Page::page_write8(&mut cpu, 0x0001, 0x42);
        Page::page_write8(&mut cpu, 0x0002, 0x00);
        Page::page_write8(&mut cpu, 0x0003, 0xB9);
        IATable::load_iat(&mut iat, &mut cpu, 0x2000);
        assert_eq!(cpu.iatr, crate::u24::u24::new(0x2000));
    }
}
fn main() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
    let mut vc: VideoKontroller2 = VideoKontroller2::new();
    Page::set_page(&mut cpu, 2); // page_no = 2(0x2000), pc = 0x2000
    Page::page_write8(&mut cpu, 0x0000, 0x2C);
    Page::page_write8(&mut cpu, 0x0001, 0x34);
    Page::page_write8(&mut cpu, 0x0002, 0x12);
    Page::page_write8(&mut cpu, 0x0003, 0x30);
    Page::page_write8(&mut cpu, 0x0004, 0x56);
    Page::page_write8(&mut cpu, 0x0005, 0x34);
    Page::page_write8(&mut cpu, 0x0006, 0x12);
    
    while cpu.memory[cpu.pc as usize] != 0 {
        KRSAssembler2::exec(&mut cpu, &mut vc);
    }
    println!("After: ");
    cpu.state();
    println!();

    // numbers by my bro
    cpu.push8(42);
    cpu.push8(67);
    cpu.push8(69);
    cpu.push8(52);
    cpu.show_stack();
    VideoKontroller2::disp(&mut vc, b"ok so\n");
    VideoKontroller2::disp(&mut vc, b"it works\n");
}




