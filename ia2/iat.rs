#[derive(Clone, Copy)]
struct IATEntry {
    num: u8,
    handler: ux::u24,
}

struct IATable {
    entries: [Option<IATEntry>; 256],
}

impl IATable {
    pub fn new() -> Self {
        Self {
            entries: [None; 256],
        }
    }

    pub fn insert_entry(&mut self, num: u8, handler: ux::u24) {
        self.entries[num as usize] = Some(IATEntry { num, handler });
    }

    /// Load IAT from memory.
    ///
    /// Layout per entry (4 bytes):
    ///   [0] num (u8)
    ///   [1] handler low  (b1)
    ///   [2] handler mid  (b2)
    ///   [3] handler high (b3)
    ///
    /// If a slot looks empty (num==0 and handler==0), we store None.
    pub fn load_iat(&mut self, cpu: &crate::cpu2::KoshkaCPU2, base: u32) {
        for num in 0u8..=u8::MAX {
            let idx = num as usize;
            let pos = base.wrapping_add((num as u32) * 4);

            let read_num = cpu.read8(pos + 0);
            let b1: ux::u24 = ux::u24::from(cpu.read8(pos + 1));
            let b2: ux::u24 = ux::u24::from(cpu.read8(pos + 2));
            let b3: ux::u24 = ux::u24::from(cpu.read8(pos + 3));
            let handler: ux::u24 = (b3 << 16) | (b2 << 8) | b1;

            let is_empty = read_num == 0 && handler == ux::u24::from(0u8);
            if is_empty {
                self.entries[idx] = None;
            } else {
                self.entries[idx] = Some(IATEntry {
                    num: read_num,
                    handler,
                });
            }
        }
    }
}