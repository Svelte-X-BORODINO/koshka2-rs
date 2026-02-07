mod cpu2;
mod video2;
mod fpu;
mod paging;
mod debug;
use paging::Page;
use crate::{cpu2::KoshkaCPU2, video2::VideoController2};


fn main() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
    VideoController2::dispd(b"CPU Initialized\n");
    cpu.memory[0x10] = b'H';
    cpu.memory[0x11] = b'e'; 
    cpu.memory[0x12] = b'l';
    cpu.memory[0x13] = b'l';
    cpu.memory[0x14] = b'o';
    Page::set_page(&mut cpu, 63);
    Page::show_page(&mut cpu, 0);
    cpu.kadv = 0x00000010 as *mut u32;
    loop { //                                                           puts:
        let byte = cpu.memory[cpu.kadv as usize]; //                    ldb ax adv
        if byte == 0 { //                                                   cmp ax $00
            break; //                                                       gz $done
        }
        print!("{}", byte as char); //                                      invoke 1
        unsafe { cpu.kadv = cpu.kadv.byte_add(1); } //               inc adv
        //                                                                  goto puts
    }
}
