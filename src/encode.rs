use std::vec;
use crate::field::FieldElement as FF;

// Algorithm 3: Converts a bit array (of a length that is a multiple of eight) into an array of bytes
pub fn bits_to_bytes(vec: Vec<u8>) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    for i in 0..vec.len() / 8 {
        let mut byte: u8 = 0;
        for j in 0..8 {
            byte = byte | (vec[i * 8 + j] << (7 - j));
        }
        bytes.push(byte);
    }
    bytes
}

// Algorithm 4: Converts an array of bytes into a bit array
pub fn bytes_to_bits(vec: Vec<u8>) -> Vec<u8> {
    let mut bits: Vec<u8> = vec![];
    for byte in vec {
        for i in 0..8 {
            bits.push((byte >> (7 - i)) & 1);
        }
    }
    bits
}

//Algorithm 5: Encodes an array of 𝑑-bit integers into a byte array for 1 ≤ 𝑑 ≤ 12.
pub fn bytes_encode(d: usize, f: Vec<u16>) -> Vec<u8> {
    let mut b = vec![0u16; 32 * d];
    for i in 0..256 {
        let mut a = f[i];
        for j in 0..d {
            b[i * d + j] = a & 1;
            a = (a - b[i * d + j]) >> 1;
        }
    }
    let bb = bits_to_bytes(b.iter().map(|&x| x as u8).collect());
    bb
}

//Algorithm 6: Decodes a byte array into an array of 𝑑-bit integers for 1 ≤ 𝑑 ≤ 12.
pub fn bytes_decode(d: usize, b: Vec<u8>) -> Vec<u16> {
    let bb: Vec<u16> = bytes_to_bits(b).iter().map(|&x| x as u16).collect();
    let mut m = 2u16.pow(d.try_into().unwrap());
    if d == 12 {
        m = FF::Q;
    }

    let mut f = vec![0u16; 256];
    for i in 0..256 {
        for j in 0..d {
            f[i] += (bb[i * d + j] * 2u16.pow(j as u32)) % m;
        }
    }
    f
}

//Algorithm 7: Encodes a polynomial 𝑓 of degree 255 into a byte array.