use crate::field::FieldElement as FF;

// Bit reverse function for 7-bit numbers
pub fn bitrev7(x: usize) -> usize {
    let mut y = 0;
    for i in 0..7 {
        y = (y << 1) | (x >> i & 1);
    }
    y
}

// Algorithm 12: Computes the product of two degree-one polynomials with respect to a quadratic modulus.
pub fn base_case_multiply(a0: FF, a1: FF, b0: FF, b1: FF, gamma: FF) -> (FF, FF) {
    let c0 = a0 * b0 + a1 * b1 * gamma;
    let c1 = a0 * b1 + a1 * b0;
    (c0, c1)
}