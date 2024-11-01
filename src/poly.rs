use std::ops::{Add, Mul};
use crate::encode::bytes_to_bits;
use crate::field::FieldElement as FF;
use crate::helper::{xof, bitrev7, base_case_multiply};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coeffs: Vec<FF>,
}

impl Polynomial{
    pub const N: usize = 256;
    pub const G: FF = FF(17);
    pub fn new(mut coeffs: Vec<FF>) -> Polynomial {
        coeffs = Polynomial::padding_zeros(coeffs);
        Polynomial { coeffs }
    }

    pub fn padding_zeros(coeffs: Vec<FF>) -> Vec<FF> {
        let mut padded_coeffs = coeffs.clone();
        while padded_coeffs.len() < Polynomial::N {
            padded_coeffs.push(FF(0));
        }
        padded_coeffs
    }

    pub fn zero_polynomial() -> Polynomial {
        Polynomial::new(vec![FF(0); Polynomial::N])
    }

    pub fn list(&self) -> Vec<u16> {
        let coeffs = self.coeffs.clone();
        coeffs.iter().map(|x| x.to_int()).collect::<Vec<_>>().to_vec()
    }
}

// Algorithm 9: Computes ̂ the NTT representation 𝑓 of the given polynomial 𝑓 ∈ 𝑅𝑞.
impl Polynomial {
    pub fn ntt(self) -> Polynomial {
        let mut f_ntt = self.clone();
        let mut i: usize = 1;
        let mut t: FF;
        for len in [128, 64, 32, 16, 8, 4, 2] {
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

// Algorithm 10: Computes ̂the polynomial 𝑓 ∈ 𝑅𝑞 that corresponds to the given NTT representation 𝑓 ∈ 𝑇𝑞.
impl Polynomial {
    pub fn intt(self) -> Polynomial {
        let mut f_intt = self.clone();
        let mut i: usize = 127;
        let mut t: FF;
        for len in [2, 4, 8, 16, 32, 64, 128] {
            for start in (0..256).step_by(2 * len) {
                let zeta = Polynomial::G.pow(bitrev7(i) as u16);
                i -= 1;
                for j in start..start + len {
                    t = f_intt.coeffs[j];
                    f_intt.coeffs[j] = t + f_intt.coeffs[j + len];
                    f_intt.coeffs[j + len] = zeta * (f_intt.coeffs[j + len] - t);
                }
            }
        }
        for i in 0..256 {
            f_intt.coeffs[i] = f_intt.coeffs[i] * FF(3303);
        }
        f_intt
    }
}

// Algorithm 11: Computes the product (in the ring 𝑇𝑞) of two NTT representations.
impl Polynomial {
    pub fn multiply_ntt(f: Polynomial, g: Polynomial) -> Polynomial {
        let mut h = Polynomial::zero_polynomial();
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

//Algorithm 7: Takes a 32-byte seed and two indices as input and outputs a pseudorandom element of 𝑇𝑞.
pub fn sample_ntt(mut bytes: Vec<u8>, i: u8, j:u8) -> Polynomial {
    bytes.push(i);
    bytes.push(j);
    assert_eq!(bytes.len(), 34);
    let mut a = Polynomial::zero_polynomial();
    let mut j = 0;
    while j < 256 {
        let c = xof(bytes.clone());
        let d1: u16 = (c[0] as u16) + 256 * ((c[1] as u16) % 16);
        let d2: u16 = (c[1] as u16) / 16 + 16 * (c[2] as u16);
        if d1 < FF::Q {
            a.coeffs[j] = FF(d1);
            j += 1;
        }
        if d2 < FF::Q && j < 256 {
            a.coeffs[j] = FF(d2);
            j += 1;
        }
    }
    a
}

// Algorithm 8: Takes a seed as input and outputs a pseudorandom sample from the distribution D𝜂(𝑅𝑞).
pub fn sample_poly_cbd(mut bytes: Vec<u16>, eta: usize) -> Vec<FF> {
    while bytes.len() < 64 * eta {
        bytes.push(0);
    }
    let mut f = vec![FF(0); 256];
    let bits = bytes_to_bits(bytes);
    for i in 0..256 {
        let mut x = 0u16;
        let mut y = 0u16;
        for j in 0..eta {
            x += bits[2 * i * eta + j];
            y += bits[(2 * i + 1) * eta + j];
            f[i] = FF(x - y);
        }
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_zeros() {
        let p = Polynomial::new(vec![FF(1), FF(2), FF(3)]);
        let padded_p = Polynomial::padding_zeros(p.coeffs);
        assert_eq!(padded_p.len(), 256);
        assert_eq!(padded_p[0], FF(1));
        assert_eq!(padded_p[1], FF(2));
        assert_eq!(padded_p[2], FF(3));
        for i in 3..256 {
            assert_eq!(padded_p[i], FF(0));
        }
    }

    #[test]
    fn test_add() {
        let p1 = Polynomial::new(vec![FF(1), FF(2), FF(3)]);
        let p2 = Polynomial::new(vec![FF(4), FF(5), FF(6)]);
        let sum = p1 + p2;
        assert_eq!(sum.coeffs[0], FF(5));
        assert_eq!(sum.coeffs[1], FF(7));
        assert_eq!(sum.coeffs[2], FF(9));
    }

    #[test]
    fn test_multiply() {
        let p1 = Polynomial::new(vec![FF(1), FF(2), FF(3)]);
        let p2 = Polynomial::new(vec![FF(4), FF(5), FF(6)]);
        let product = p1 * p2;
        assert_eq!(product.coeffs[0], FF(4));
        assert_eq!(product.coeffs[1], FF(13));
        assert_eq!(product.coeffs[2], FF(28));
        assert_eq!(product.coeffs[3], FF(27));
        assert_eq!(product.coeffs[4], FF(18));
    }
}