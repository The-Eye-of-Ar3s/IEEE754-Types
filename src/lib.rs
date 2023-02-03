#![allow(non_camel_case_types)]
pub mod binary;

#[cfg(test)]
mod tests {
    use super::binary::{Binary32};

    #[test]
    fn binary_32_into_f32() {
        let result: f32 = Binary32::default().into();
        let expected: f32 = 1_f32;
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_32_from_f32() {
        let result: Binary32 = Binary32::from(1_f32);
        let expected: Binary32 = Binary32::default();
        assert_eq!(result, expected);
    }
}