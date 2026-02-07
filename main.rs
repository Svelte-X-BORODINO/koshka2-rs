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
    let res = cpu.fpu.add(5.3, 6.4);
    println!(res);
}
