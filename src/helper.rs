use crate::field::FieldElement as FF;
use sha3::{Shake128, Shake256, Sha3_256, Sha3_512, Digest, digest::{Update, ExtendableOutput, XofReader}};
use rand::Rng;

// Compress/Decompress function
fn field_reduce(x: u16) -> u16 {
    let wx = x.wrapping_sub(3329);
    wx.wrapping_add((wx >> 15).wrapping_mul(3329))
}

fn div_and_round(dividend: u32, divisor: u32) -> u16 {
    field_reduce(((dividend + (divisor >> 1)) / divisor) as u16)
}

pub fn compress(mut v: Vec<u16>, d: u8) -> Vec<u16> {
    for i in 0..v.len() {
        v[i] = div_and_round((v[i] as u32) << d, 3329);
    }
    v
}

pub fn decompress(mut v: Vec<u16>, d: u8) -> Vec<u16> {
    for i in 0..v.len() {
        v[i] = div_and_round((v[i] as u32) * 3329, 1u32 << d);
    }
    v
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
    (output[0..32].to_vec(), output[32..64].to_vec())
}

// Generate random bytes
pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen::<u8>()).collect()
}
