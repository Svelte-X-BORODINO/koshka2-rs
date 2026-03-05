mod cpu2;
mod video2;
mod paging;
mod debug;
mod ia2;
mod asm;
mod state;

use crate::{cpu2::KoshkaCPU2, paging::Page, asm::asm2::KRSAssembler2};

#[test]
fn kadv() {
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

#[test]
fn showpage() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
    Page::set_page(&mut cpu, 16);
    Page::show_page(&mut cpu, 0);
}
#[test]
fn showstate() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
    cpu.state();
}
fn main() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
    // mov ax $42
    Page::page_write8(&mut cpu, 0x2000, 0xB0);
    Page::page_write8(&mut cpu, 0x2001, 0x42);
    
    KRSAssembler2::exec(&mut cpu);
    println!("After: ");
    cpu.state();
    println!();
}




