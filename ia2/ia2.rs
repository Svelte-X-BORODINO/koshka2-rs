use crate::cpu2::KoshkaCPU2;

pub type InvokeFunction = fn(&mut KoshkaCPU2);
pub type IAT = HashMap<u8, InvokeFunction>;


pub struct InvokeAction2;

pub fn putc(cpu: &mut KoshkaCPU2) {
    print!("{}", cpu.memory[cpu.kadv as usize] as char);
}

pub fn readc(cpu: &mut KoshkaCPU2) {
    io::stdin().read_exact(&mut cpu.k[BX])
}

impl InvokeAction2 {
    pub fn init() -> IAT {
        let mut table: IAT;
        table.insert(1, putc as InvokeFunction);
        table.insert(2, readc as InvokeFunction);
        table
    }

    pub fn invoke(ia_table: &IAT, num: u8) {
        
    }
}