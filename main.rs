mod cpu2;
mod video2;
mod paging;
mod debug;
mod ia2;
mod asm;

use crate::cpu2::KoshkaCPU2;


fn main() {
    let cpu: KoshkaCPU2 = KoshkaCPU2::new();
}
