use std::ops::{Add, Mul};

pub trait ReadNum: From<u8> + Mul<Self, Output = Self> + Add<Self, Output = Self> + Copy {
    const ZERO: Self;
    const ONE: Self;
    const TEN: Self;
}

macro_rules! impl_readnum {
    ($(
        [$($ty:ty),+$(,)?]: $zero:literal, $one:literal, $ten:literal;
    )+) => {
        $(
            $(
                impl ReadNum for $ty {
                    const ZERO: Self = $zero;
                    const ONE: Self = $one;
                    const TEN: Self = $ten;
                }
            )+
        )+
    };
}

impl_readnum! {
    [u8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize]: 0, 1, 10;
    [f32, f64]: 0.0, 1.0, 10.0;
}

#[inline]
pub fn readnum<T: ReadNum, const END: u8>(input: &[u8], cursor: &mut usize) -> T {
    let mut n = T::ZERO;
    while input[*cursor] != END {
        n = n * T::TEN + (input[*cursor] - b'0').into();
        *cursor += 1;
    }
    n
}

#[inline]
pub fn readnum_rev<T: ReadNum + std::fmt::Display, const END: u8>(
    input: &[u8],
    cursor: &mut usize,
) -> T {
    let mut n = T::ZERO;
    let mut mul = T::ONE;
    while input[*cursor] != END {
        n = n + mul * (input[*cursor] - b'0').into();
        mul = mul * T::TEN;
        *cursor -= 1;
    }
    n
}

#[inline]
pub fn readnum_12<T: ReadNum + std::fmt::Display, const END: u8>(
    input: &[u8],
    cursor: &mut usize,
) -> T {
    let first = (input[*cursor] - b'0').into();
    let next = input[*cursor + 1];
    if next == END {
        *cursor += 1;
        first
    } else {
        *cursor += 2;
        first * T::TEN + (next - b'0').into()
    }
}
