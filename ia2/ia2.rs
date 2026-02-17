//use std::io::Read;
//
//use crate::cpu2::{KoshkaCPU2, AX, BX, CX, DX};
//use std::collections::HashMap;
//
//pub type InvokeFunction = fn(&mut KoshkaCPU2) -> Result<(), std::io::Error>;
//pub type IAT = HashMap<u8, InvokeFunction>;
//
//
//pub struct InvokeAction2;
//
//pub fn putc(cpu: &mut KoshkaCPU2) -> Result<(), std::io::Error> {
//    print!("{}", cpu.memory[cpu.kadv as usize] as char);
//    Ok(())
//}
//
//pub fn readc(cpu: &mut KoshkaCPU2) -> Result<(), std::io::Error> {
//    std::io::stdin().read_exact(cpu.k[BX]); // expected &mut [u8], found u16
//    Ok(())
//}
//
//impl InvokeAction2 {
//    pub fn init() -> IAT {
//        let mut table: IAT;
//        table.insert(1, putc as InvokeFunction);
//        table.insert(2, readc as InvokeFunction);
//        table
//    }
//    
//    pub fn invoke(ia_table: &IAT, num: u8) {
//        
//    }
//}