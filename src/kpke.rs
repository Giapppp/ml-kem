use crate::encode::*;
use crate::field::FieldElement as FF;
use crate::helper::*;
use crate::matrix::*;
use crate::poly::*;
use crate::constant::{K, ETA1, ETA2, DU, DV};

// Algorithm 13: Uses randomness to generate an encryption key and a corresponding decryption key.
pub fn kpke_key_gen(mut bytes: Vec<u8>) -> (Vec<u16>, Vec<u16>) {
    bytes.push(K as u8);
    let (rho, sigma) = g(bytes);
    let mut n = 0;

    let mut a = Matrix::zero_matrix(K, K);
    let mut s = Vec::new();
    let mut e = Vec::new();

    for i in 0..K {
        for j in 0..K {
            a.matrix[i][j] = sample_ntt(rho.clone(), j as u8, i as u8);
        }
    }

    for _ in 0..K {
        s.push(Polynomial::new(sample_poly_cbd(prf(ETA1, sigma.clone(), n), ETA1)));
        n += 1;
    }

    for _ in 0..K {
        e.push(Polynomial::new(sample_poly_cbd(prf(ETA1, sigma.clone(), n), ETA1)));
        n += 1;
    }

    for i in 0..K {
        s[i] = s[i].clone().ntt();
        e[i] = e[i].clone().ntt();
    }

    let t = add(mul(&a, s.clone()), e.clone());
    
    let mut ek_pke = Vec::new();
    for i in 0..K {
        let temp: Vec<u16> = bytes_encode(12, t[i].coeffs.iter().map(|x| x.to_int()).collect());
        ek_pke.extend(temp);
    }
    ek_pke.extend(rho.iter().map(|x| *x as u16));

    let mut pk_pke = Vec::new();
    for i in 0..K {
        let temp: Vec<u16> = bytes_encode(12, s[i].coeffs.iter().map(|x| x.to_int()).collect());
        pk_pke.extend(temp);
    }

    (ek_pke, pk_pke)
}

// Algorithm 14: Uses the encryption key to encrypt a plaintext message using the randomness 𝑟.
pub fn kpke_enc(ek_pke: Vec<u16>, m: Vec<u16>, r: Vec<u8>) -> Vec<u16> {
    let mut n = 0;
    let mut t: Vec<Polynomial> = Vec::new();

    for i in 0..K {
        let temp: Polynomial = Polynomial::new(bytes_decode(12, ek_pke.clone()[i * 32..(i + 1) * 32].to_vec()).iter().map(|x| FF(*x)).collect());
        t.push(temp);
    }

    let rho: Vec<u8> = ek_pke.clone()[384 * K..384 * K + 32].iter().map(|x| *x as u8).collect();

    let mut a = Matrix::zero_matrix(K, K);
    let mut y = Vec::new();
    let mut e1 = Vec::new();
    let mut u = Vec::new();

    for i in 0..K {
        for j in 0..K {
            a.matrix[i][j] = sample_ntt(rho.clone(), j as u8, i as u8);
        }
    }

    for _ in 0..K {
        y.push(Polynomial::new(sample_poly_cbd(prf(ETA1, r.clone(), n), ETA1)).ntt());
        n += 1;
    }

    for _ in 0..K {
        e1.push(Polynomial::new(sample_poly_cbd(prf(ETA2, r.clone(), n), ETA2)).ntt());
        n += 1;
    }

    let e2 = Polynomial::new(sample_poly_cbd(prf(ETA2, r.clone(), n), ETA2));

    for i in 0..K {
        y[i] = y[i].clone().ntt();
    }

    let ay = mul(&a.transpose(), y.clone());
    for i in 0..K {
        u.push(ay[i].clone().intt() + e1[i].clone());
    }

    let nuy = Polynomial::new(decompress(bytes_decode(1, m), 1).iter().map(|x| FF(*x)).collect());
    
    let v = vec_mul(t.clone(), y.clone()).intt() + e2.clone() + nuy.clone();
    
    let mut c1: Vec<u16> = Vec::new();
    for i in 0..K {
        let temp: Vec<u16> = bytes_encode(DU, compress(u[i].coeffs.iter().map(|x| x.to_int()).collect(), DU as u8));
        c1.extend(temp);
    }

    let c2 = bytes_encode(DV, compress(v.coeffs.iter().map(|x| x.to_int()).collect(), DV as u8));
    c1.extend(c2);
    c1
}

// Algorithm 15: Uses the decryption key to decrypt a ciphertext.
pub fn kpke_dec(pk_pke: Vec<u16>, c: Vec<u16>) -> Vec<u16> {
    let c1 = c.clone()[0..32 * DU * K].to_vec();
    let c2 = c.clone()[32 * DU * K..32 * (DU * K + DV)].to_vec();

    let mut u = Vec::new();
    for i in 0..K {
        u.push(Polynomial::new(decompress(bytes_decode(DU, c1.clone()[32 * DU * i..32 * DU * (i + 1)].to_vec()), DU as u8).iter().map(|x| FF(*x)).collect()));
    }

    let v = Polynomial::new(decompress(bytes_decode(DV, c2), DV as u8).iter().map(|x| FF(*x)).collect());
    let mut s: Vec<Polynomial> = Vec::new();
    
    for i in 0..K {
        s.push(Polynomial::new(bytes_decode(12, pk_pke.clone()[32 * i..32 * (i + 1)].to_vec()).iter().map(|x| FF(*x)).collect()));
    }

    for i in 0..K {
        u[i] = u[i].clone().ntt();
    }

    let w = v - vec_mul(s.clone(), u.clone()).intt();
    let m = bytes_encode(1, compress(w.coeffs.iter().map(|x| x.to_int()).collect(), 1));
    m
}
