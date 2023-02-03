use std::{fmt::Display};

use bit::BitIndex;
use bitvec::{prelude::{
    BitArr,
    Lsb0, BitArray,
}, bitarr, vec::BitVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binary32(BitArr!(for 32, in u32, Lsb0));


impl Default for Binary32 {
    fn default() -> Self {
        return Binary32(bitarr![u32, Lsb0; 0,0,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])
    }
}

impl Display for Binary32 {
    /// 9 decimal digits for Binary32
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.9}", Into::<f32>::into(*self))
    }
}

impl Into<f32> for Binary32 {
    fn into(self) -> f32 {
        let (part1, mantissa_bits) = self.0.split_at(9);
        let sign = *part1.get(0).unwrap();
        let (_, exponent_bits) = part1.split_at(1);
        let mut s: f32 = 1_f32;
        if sign {
            s = -1_f32;
        }
        let mut exponent: i32 = -127_i32;
        for i in 1..9 {
            match *exponent_bits.get(8-i).unwrap() {
                false => {}
                true => {
                    exponent += 2_i32.pow(i as u32 -1)
                }
            }
        }
        let mut mantissa: f32 = 1_f32;
        for i in 0..23 {
            match *mantissa_bits.get(i).unwrap() {
                false => {}
                true => {
                    mantissa += 2_f32.powi(-1_i32 * (i + 1) as i32)
                }
            }
        }
        return s*mantissa*2_f32.powi(exponent);
    }
}

impl From<f32> for Binary32 {
    fn from(value: f32) -> Self {
        let mut data = bitarr!(u32, Lsb0; 32; 1);
        data.set(0, value.is_sign_negative());
        let integral = value.abs().trunc() as u32;
        let mut integral_bits: BitVec<u8, Lsb0>= (0..32).map(|i| integral.bit(i)).collect::<BitVec<u8, Lsb0>>();
        for _ in 0..integral_bits.trailing_zeros() {
            integral_bits.pop();
        };

        integral_bits.reverse();
        
        let mut fractional: f32 = value.abs().fract();
        let mut fractional_bits: BitVec<u8, Lsb0> = BitVec::new();
        while fractional_bits.len() != 23 {
            fractional *= 2_f32;
            if fractional == 1.0 {
                fractional_bits.push(true);
                break;
            } else if fractional > 1.0 {
                fractional_bits.push(true);
                fractional -= 1.0;
            } else {
                fractional_bits.push(false);
            }
        }

        if fractional_bits.count_ones() == 0 && integral_bits.len() == 0 {
            return Binary32(BitArray::ZERO);
        }

        let mut exponent:u8 = 0;
        if integral_bits.len() > 0 {
            exponent = (126 + integral_bits.len()) as u8;
            integral_bits.remove(0);
        } else if fractional_bits.len() > 0 {
            exponent = (126 - fractional_bits.leading_zeros()) as u8;
            for _ in 0..fractional_bits.leading_zeros()+1 {
                fractional_bits.remove(0);
            }
        }

        let exponent_bits = (0..8).rev().map(|i| exponent.bit(i)).collect::<BitVec<u8, Lsb0>>();

        let mut mantissa = integral_bits.clone();
        mantissa.extend(fractional_bits);
        mantissa.truncate(23);
        while mantissa.len() < 23 {
            mantissa.push(false);
        }

        for i in 0..8 {
            data.set(i+1, *exponent_bits.get(i).unwrap())
        }

        for i in 0..23 {
            data.set(i+9, *mantissa.get(i).unwrap())
        }

        return Binary32(data);
    }
}