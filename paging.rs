use crate::cpu2::KoshkaCPU2;

pub struct Page;

macro_rules! do_set_page {
    ($cpu:expr, $num:expr) => {
        $cpu.current_page = $num
    };
    () => {
        compile_error!("Invalid do_set_page!");
    }
}

impl Page {
    pub fn set_page(cpu: &mut KoshkaCPU2, page_no: u8) { if page_no > 63 { cpu.panic_cpu("invalid_page_number") }; do_set_page!(cpu, page_no) }

    /// x86 physical address = segment << 4 + offset,
    /// but in KRSII: physical address = current_page << 12 + ofs
    fn paddr(cpu: &mut KoshkaCPU2, ofs: u32) -> u32 {
        ((cpu.current_page as u32) << 12) + ofs
    }

    /// Show 4096 bytes of page.
    pub fn show_page(cpu: &mut KoshkaCPU2, ofs: u32) {
        let start = Self::paddr(cpu, ofs);
        let end = start + 4096;
        cpu.show_mem(start as usize, end as usize);
    }

    /// Read 1 byte from page.
    pub fn page_read8(cpu: &mut KoshkaCPU2, ofs: u32) -> u8 {
        if ofs > 0xFFF { cpu.panic_cpu("ofs_overflow"); }
        if Self::paddr(cpu, ofs) > 0x3FFFF { cpu.panic_cpu("paddr_overflow") }
        cpu.memory[Self::paddr(cpu, ofs) as usize]
    }

    /// Write 1 byte to page.
    pub fn page_write8(cpu: &mut KoshkaCPU2, ofs: u32, data: u8) {
        if ofs > 0xFFF { cpu.panic_cpu("ofs_overflow"); }
        if Self::paddr(cpu, ofs) > 0x3FFFF { cpu.panic_cpu("paddr_overflow") }
        cpu.memory[Self::paddr(cpu, ofs) as usize] = data;
    }
}
