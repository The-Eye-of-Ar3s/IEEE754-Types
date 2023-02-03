use std::fmt::Display;

use bitvec::{prelude::{
    BitArr,
    Lsb0
}, bitarr};

#[derive(Debug, Clone, Copy)]
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
        for i in 0..8 {
            match *exponent_bits.get(i).unwrap() {
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
                    mantissa += 2_f32.powi(-1_i32 * i as i32)
                }
            }
        }
        //println!("SIGN: {} {}", sign, s);
        //println!("EXP: {} {}", exponent_bits, exponent);
        //println!("MAN: {} {}", mantissa_bits, mantissa);
        //println!("{s}*{mantissa}*2^{exponent}");
        return s*mantissa*2_f32.powi(exponent);
    }
}