use std::vec;
use crate::field::FieldElement as FF;

// Algorithm 3: Converts a bit array (of a length that is a multiple of eight) into an array of bytes in little endian order.
pub fn bits_to_bytes(mut bits: Vec<u16>) -> Vec<u16> {
    while bits.len() % 8 != 0 {
        bits.push(0);
    }
    let l = bits.len();
    let mut bytes: Vec<u16> = vec![0; l / 8];
    for i in 0..l {
        bytes[i / 8] = bytes[i / 8] + (bits[i] << (i % 8)); 
    }
    bytes
}

// Algorithm 4: Converts an array of bytes in little endian into a bit array
pub fn bytes_to_bits(mut bytes: Vec<u16>) -> Vec<u16> {
    let l = bytes.len();
    let mut bits = vec![0; l * 8];
    for i in 0..l {
        for j in 0..8 {
            bits[i * 8 + j] = bytes[i] & 1;
            bytes[i] = bytes[i] >> 1;
        }
    }
    bits
}

//Algorithm 5: Encodes an array of 𝑑-bit integers into a byte array for 1 ≤ 𝑑 ≤ 12.
pub fn bytes_encode(d: usize, mut f: Vec<u16>) -> Vec<u16> {
    while f.len() != 256 {
        f.push(0);
    }
    let mut b = vec![0u16; 256 * d];
    for i in 0..256 {
        let mut a = f[i];
        for j in 0..d {
            b[i * d + j] = a & 1;
            a = (a - b[i * d + j]) >> 1;
        }
    }
    bits_to_bytes(b)
}

//Algorithm 6: Decodes a byte array into an array of 𝑑-bit integers for 1 ≤ 𝑑 ≤ 12.
pub fn bytes_decode(d: usize, mut bytes: Vec<u16>) -> Vec<u16> {
    let mut m = 1 << d;
    if d == 12 {
        m = FF::Q;
    }

    while bytes.len() != 32 * d {
        bytes.push(0);
    }

    let bit = bytes_to_bits(bytes);
    assert_eq!(bit.len(), 256 * d);
    let mut f = vec![0u16; 256];
    for i in 0..256 {
        for j in 0..d {
            f[i] += bit[i * d + j] << j;
        }
        f[i] %= m;
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_bytes() {
        let vec = vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let bytes = bits_to_bytes(vec);
        assert_eq!(bytes, vec![85, 128]);
    }

    #[test]
    fn test_bytes_to_bits() {
        let vec = vec![85, 128];
        let bits = bytes_to_bits(vec);
        assert_eq!(bits, vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_bytes_encode() {
        let d = 5;
        let f = vec![0b11110, 0b10100, 0b11000, 0b10010, 0b11101];
        let bytes = bytes_encode(d, f);
        let mut check = vec![0b10011110, 0b1100010, 0b11011001, 0b00000001];
        check.append(&mut vec![0; 32*d - check.len()]);
        assert_eq!(bytes, check);
    }

    #[test]
    fn test_bytes_decode() {
        let d = 4;
        let bytes = vec![0b00010010, 0b00110100, 0b01010110, 0b01111000];
        let f = bytes_decode(d, bytes);
        let mut check: Vec<u16> = vec![2, 1, 4, 3, 6, 5, 8, 7];
        check.append(&mut vec![0; 256 - check.len()]);
        assert_eq!(f, check);
    }
}