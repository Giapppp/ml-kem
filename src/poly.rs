use std::ops::{Add, Mul};
use crate::field::FieldElement;
use crate::helper::{bitrev7, base_case_multiply};

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coeffs: Vec<FieldElement>,
}

impl Polynomial{
    pub const N: usize = 256;
    pub const G: FieldElement = FieldElement(17);
    pub fn new(coeffs: Vec<FieldElement>) -> Polynomial {
        Polynomial { coeffs }
    }

    pub fn padding_zeros(coeffs: Vec<FieldElement>) -> Vec<FieldElement> {
        let mut padded_coeffs = coeffs.clone();
        while padded_coeffs.len() < Polynomial::N {
            padded_coeffs.push(FieldElement(0));
        }
        padded_coeffs
    }
}

// Algorithm 9
impl Polynomial {
    fn ntt(self) -> Polynomial {
        let mut f_ntt = self.clone();
        let mut i: usize = 1;
        let mut t: FieldElement;
        for len in [2, 4, 8, 16, 32, 64, 128] {
            for start in (0..256).step_by(2 * len) {
                let zeta = Polynomial::G.pow(bitrev7(i) as u16);
                i += 1;
                for j in start..start + len {
                    t = zeta * f_ntt.coeffs[j + len];
                    f_ntt.coeffs[j + len] = f_ntt.coeffs[j] - t;
                    f_ntt.coeffs[j] = f_ntt.coeffs[j] + t;
                }
            }
        }
        f_ntt
    }
}

// Algorithm 10
impl Polynomial {
    fn intt(self) -> Polynomial {
        let mut f_intt = self.clone();
        let mut i: usize = 1;
        let mut t: FieldElement;
        for len in [128, 64, 32, 16, 8, 4, 2] {
            for start in (0..256).step_by(2 * len) {
                let zeta = Polynomial::G.pow(FieldElement::Q - bitrev7(i) as u16);
                i += 1;
                for j in start..start + len {
                    t = zeta * f_intt.coeffs[j + len];
                    f_intt.coeffs[j + len] = f_intt.coeffs[j] - t;
                    f_intt.coeffs[j] = f_intt.coeffs[j] + t;
                }
            }
        }
        for i in 0..256 {
            f_intt.coeffs[i] = f_intt.coeffs[i] * FieldElement(256).inv();
        }
        f_intt
    }
}

// Algorithm 11
impl Polynomial {
    fn multiply_ntt(f: Polynomial, g: Polynomial) -> Polynomial {
        let mut h = Polynomial::new(vec![FieldElement(0); Polynomial::N]);
        for i in 0..128 {
            let exp = (2 * bitrev7(i) + 1) as u16;
            let coeffs = base_case_multiply(f.coeffs[2 * i], f.coeffs[2 * i + 1], g.coeffs[2 * i], g.coeffs[2 * i + 1], Polynomial::G.pow(exp));
            h.coeffs[2 * i] = coeffs.0;
            h.coeffs[2 * i + 1] = coeffs.1;
        }
        h
    }
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        let mut sum = Vec::new();
        for i in 0..Polynomial::N {
            sum.push(self.coeffs[i] + other.coeffs[i]);
        }
        Polynomial::new(sum)
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let f_ntt = self.ntt();
        let g_ntt = other.ntt();
        let h_ntt = Polynomial::multiply_ntt(f_ntt, g_ntt);
        h_ntt.intt()
    }
}