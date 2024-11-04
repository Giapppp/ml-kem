use std::ops::{Add, Sub, Mul};
use std::cmp::{PartialEq, Eq};
pub type Integer = u16;
use crate::constant::Q;

#[derive(Debug, Clone, Copy)]
pub struct FieldElement(pub Integer);

impl FieldElement {
    pub fn new(value: Integer) -> FieldElement {
        FieldElement(value % Q)
    }

    pub fn pow(&self, exponent: Integer) -> FieldElement {
        let mut ans = FieldElement(1);
        let mut base = FieldElement(self.0);
        let mut exp = exponent;
        while exp > 0 {
            if exp & 1 == 1 {
                ans = ans * base;
            }
            base = base * base;
            exp >>= 1;
        }
        ans
    }

    pub fn inv(&self) -> FieldElement {
        self.pow(Q - 2)
    }

    pub fn to_int(&self) -> Integer {
        self.0
    }
}   // End of FieldElement struct

impl Add<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        FieldElement((self.0 + other.0) % Q)
    }
}

impl Sub<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        FieldElement((self.0 + Q - other.0) % Q)
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        let _q = Q as u32;
        let _ans = ((self.0 as u32) * (other.0 as u32)) % _q;
        FieldElement(_ans as Integer)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.0 == other.0
    }
}

impl Eq for FieldElement {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = FieldElement(2);
        let b = FieldElement(15);
        let c = FieldElement(17);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement(2);
        let b = FieldElement(15);
        let c = FieldElement(3316);
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement(17);
        let b = FieldElement(1175);
        let c = FieldElement(1);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_pow() {
        let a = FieldElement(3);
        let b = a.pow(123);
        assert_eq!(b, FieldElement(838));
    }

    #[test]
    fn test_inv() {
        let a = FieldElement(17);
        let b = a.inv();
        assert_eq!(b, FieldElement(1175));
    }
}