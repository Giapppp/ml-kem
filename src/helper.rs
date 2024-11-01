use crate::field::FieldElement as FF;
use sha3::{Shake128, Shake256, Sha3_256, Sha3_512, Digest, digest::{Update, ExtendableOutput, XofReader}};

// Bit reverse function for 7-bit numbers
pub fn bitrev7(x: usize) -> usize {
    let mut y = 0;
    for i in 0..7 {
        y = (y << 1) | (x >> i & 1);
    }
    y
}

// Compress/Decompress function
pub fn compress(v: Vec<u16>, d: u8) -> Vec<u16> {
    v.iter()
    .map(|x| ((((1u16 << d) as f32 / FF::Q as f32) * (*x as f32)).round() as u16) & ((1u16 << d - 1)))
    .collect()
}

pub fn decompress(v: Vec<u16>, d: u8) -> Vec<u16> {
    v.iter()
    .map(|x| ((FF::Q as f32 / (1u16 << d) as f32) * (*x as f32))
    .round() as u16).collect()
}

// Algorithm 12: Computes the product of two degree-one polynomials with respect to a quadratic modulus.
pub fn base_case_multiply(a0: FF, a1: FF, b0: FF, b1: FF, gamma: FF) -> (FF, FF) {
    let c0 = a0 * b0 + a1 * b1 * gamma;
    let c1 = a0 * b1 + a1 * b0;
    (c0, c1)
}

// XOF function
pub fn xof(input: Vec<u8>) -> Vec<u16> {
    let mut xof = Shake128::default();
    xof.update(&input);
    let mut output = vec![0u8; 3];
    xof.finalize_xof().read(&mut output);
    output.iter().map(|x| *x as u16).collect()
}

// PRF function
pub fn prf(eta: usize, mut s: Vec<u8>, b: u8) -> Vec<u16> {
    let size = 64 * eta;
    s.push(b);
    let mut shake256 = Shake256::default();
    shake256.update(&s);
    let mut output = vec![0u8; size];
    shake256.finalize_xof().read(&mut output);
    output.iter().map(|x| *x as u16).collect()
}

// Some hash functions
pub fn h(s: Vec<u8>) -> Vec<u8> {
    let mut sha3 = Sha3_256::new();
    Update::update(&mut sha3, &s);
    sha3.finalize().to_vec()
}

pub fn j(s: Vec<u8>) -> Vec<u8> {
    let mut shake256 = Shake256::default();
    shake256.update(&s);
    let mut output = vec![0u8; 32];
    shake256.finalize_xof().read(&mut output);
    output
}

pub fn g(s: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut sha3 = Sha3_512::new();
    Update::update(&mut sha3, &s);
    let output = sha3.finalize().to_vec();
    (output[..32].to_vec(), output[32..].to_vec())
}

