mod cpu2;
mod video2;
mod paging;
mod debug;
mod ia2;
mod asm;
use paging::Page;
use crate::{cpu2::KoshkaCPU2, video2::VideoController2};


fn main() {
    let mut cpu: KoshkaCPU2 = KoshkaCPU2::new();
}
