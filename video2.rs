use core::arch::asm;
/// Lutiy pizdec.
// ya pomny
// hotel na STM32 prisoedenit' RX -> RX i TX -> TX k CH340G
// no eto ploho
// ne delaite tak

#[derive(Clone)]
pub struct VideoKontroller2 {
    tx: u8,
    rx: u8,
}

impl VideoKontroller2 {
    pub fn new() -> Self {
        Self {
            tx: 0,
            rx: 0,
        }
    }
    
    fn send(&mut self, b: u8) {
        self.tx = b;
        self.rx = self.tx;
    }

    fn recv(&mut self) -> u8 {
        self.rx
    }

    fn putc(&mut self) {
        unsafe {
            asm!(
                "syscall",
                in("rax") 1,
                in("rdi") 1,
                in("rsi") &self.rx as *const u8,
                in("rdx") 1
            );
        }
    }

    pub fn disp(&mut self, s: &[u8]) {
        for &c in s {
            self.send(c); 
            self.putc();
        }
    }

    pub fn dispd(&mut self, s: &str) {
        Self::disp(self, b"[*]");
        Self::disp(self, s.as_bytes());
    }
}
