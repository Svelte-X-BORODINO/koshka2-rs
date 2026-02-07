use std::io::Write;
use std::arch::asm;

pub struct VideoController2;

impl VideoController2 {
    pub fn new(&self) -> Self {
        Self {}
    }
    fn do_putc(c: u8) {
        print!("{}", c as char);
        let _ = std::io::stdout().flush(); 
    }

    pub fn putc(c: u8) {
        Self::do_putc(c);
    }

    pub fn disp(s: &[u8]) {
        for c in s {
            Self::putc(*c);
            if *c == b'\n' {
                Self::putc(b'\r');
            }
        }
    } 

    pub fn dispd(s: &[u8]) {
        Self::disp(b"[*] ");
        for c in s {
            Self::putc(*c);
        }
    } 
}
