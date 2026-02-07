use crate::cpu2::KoshkaCPU2;

pub struct Page;

macro_rules! do_set_page {
    ($cpu:expr, $num:expr) => {
        $cpu.current_page = $num
    };
    () => {
        compile_error!("Invalid do_set_page!"); // да я знаю про эту имбу
        // это же как zig-comptime
    }
}

impl Page {
    pub fn set_page(cpu: &mut KoshkaCPU2, page_no: u8) { if page_no > 63 { cpu.panic_cpu("invalid_page_number") }; do_set_page!(cpu, page_no) }

    fn get_paddr(cpu: &mut KoshkaCPU2, ofs: u32) -> u32 {
        4096 * cpu.current_page as u32 + ofs
    }

    pub fn show_page(cpu: &mut KoshkaCPU2, ofs: u32) {
        let start = Self::get_paddr(cpu, ofs);
        let end = start + 4096;
        cpu.show_mem(start as usize, end as usize);
    }

    pub fn page_read8(cpu: &mut KoshkaCPU2, ofs: u32) -> u8 {
        if ofs > 0x3FFFF { cpu.panic_cpu("ofs_overflow"); }
        cpu.memory[Self::get_paddr(cpu, ofs) as usize]

    }

    pub fn page_write8(cpu: &mut KoshkaCPU2, ofs: u32, data: u8) {
        if ofs > 0x3FFFF { cpu.panic_cpu("ofs_overflow"); }
        cpu.memory[Self::get_paddr(cpu, ofs) as usize] = data;
    }
}
