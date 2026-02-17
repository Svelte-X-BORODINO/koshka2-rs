//use crate::cpu2::KoshkaCPU2;
//
//pub struct KoshkaFPU {
//    fpr: [half::f16; 16]
//}
//
//impl KoshkaFPU {
//    pub fn new() -> Self {
//        KoshkaFPU {
//            fpr: [half::f16::from_f32(0.0); 16]
//        }
//    }
//
//    pub fn add(&self, a: half::f16, b: half::f16) -> half::f16 {
//        a + b
//    }
//
//    pub fn sub(&self, a: half::f16, b: half::f16) -> half::f16 {
//        a - b
//    }
//    
//    pub fn mul(&self, a: half::f16, b: half::f16) -> half::f16 {
//        a * b
//    }
//
//    pub fn div(&self, a: half::f16, b: half::f16) -> half::f16 {
//        a / b
//    }
//}