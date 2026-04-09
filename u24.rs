#![no_std]
use core::ops::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct u24(u32);

impl u24 {
    pub const MAX: Self = Self(0xFFFFFF);
    pub const MIN: Self = Self(0x000000);

    pub fn new(n: u32) -> Self {
        Self(n & 0xFFFFFF)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

impl From<u8> for u24 {
    #[inline]
    fn from(v: u8) -> Self { Self::new(v as u32) }
}

impl From<u16> for u24 {
    #[inline]
    fn from(v: u16) -> Self { Self::new(v as u32) }
}

impl From<u24> for u32 {
    #[inline]
    fn from(v: u24) -> Self { v.0 }
}

impl From<u24> for u64 {
    #[inline]
    fn from(v: u24) -> Self { v.0 as u64 }
}

impl Add for u24 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self { Self::new(self.0 + rhs.0) }
}

impl Add<u32> for u24 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u32) -> Self { Self::new(self.0 + rhs) }
}

impl Sub for u24 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self { Self::new(self.0.wrapping_sub(rhs.0)) }
}

impl Sub<u32> for u24 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u32) -> Self { Self::new(self.0.wrapping_sub(rhs)) }
}

impl Mul for u24 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self { Self::new(self.0 * rhs.0) }
}

impl Mul<u32> for u24 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u32) -> Self { Self::new(self.0 * rhs) }
}

impl Div for u24 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 { Self::MAX } else { Self::new(self.0 / rhs.0) }
    }
}

impl Div<u32> for u24 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u32) -> Self {
        if rhs == 0 { Self::MAX } else { Self::new(self.0 / rhs) }
    }
}

impl BitAnd for u24 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self::new(self.0 & rhs.0) }
}

impl BitAnd<u32> for u24 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: u32) -> Self { Self::new(self.0 & rhs) }
}

impl BitOr for u24 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self::new(self.0 | rhs.0) }
}

impl BitOr<u32> for u24 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: u32) -> Self { Self::new(self.0 | rhs) }
}

impl BitXor for u24 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self::new(self.0 ^ rhs.0) }
}

impl BitXor<u32> for u24 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: u32) -> Self { Self::new(self.0 ^ rhs) }
}

impl Not for u24 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self { Self::new(!self.0 & 0xFFFFFF) }
}

impl Shl<usize> for u24 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: usize) -> Self { Self::new(self.0 << rhs) }
}

impl Shr<usize> for u24 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: usize) -> Self { Self::new(self.0 >> rhs) }
}