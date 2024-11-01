use crate::encode::*;
use crate::field::FieldElement as FF;
use crate::helper::*;
use crate::matrix;
use crate::matrix::*;
use crate::poly::*;

// Algorithm 13: Uses randomness to generate an encryption key and a corresponding decryption key.
pub fn kpke_key_gen(mut bytes: Vec<u8>, k: usize, eta: usize) -> (Vec<u16>, Vec<u16>) {
    bytes.push(k as u8);
    let (rho, sigma) = g(bytes);
    let mut n = 0;

    let mut a = matrix::Matrix::zero_matrix(k, k);
    let mut s = vec![Polynomial::zero_polynomial(); k];
    let mut e = vec![Polynomial::zero_polynomial(); k];

    for i in 0..k {
        for j in 0..k {
            a.matrix[i][j] = sample_ntt(rho.clone(), j as u8, i as u8);
        }
    }

    for i in 0..k {
        s[i] = Polynomial::new(sample_poly_cbd(prf(eta, sigma.clone(), n), eta)).ntt();
        n += 1;
    }

    for i in 0..k {
        e[i] = Polynomial::new(sample_poly_cbd(prf(eta, sigma.clone(), n), eta)).ntt();
        n += 1;
    }

    let t = matrix::add(matrix::mul(&a, s.clone()), e.clone());
    
    let mut ek_pke = Vec::new();
    for i in 0..k {
        let temp: Vec<u16> = bytes_encode(12, t[i].coeffs.iter().map(|x| x.to_int()).collect());
        ek_pke.extend(temp);
    }
    ek_pke.extend(rho.iter().map(|x| *x as u16));

    let mut pk_pke = Vec::new();
    for i in 0..k {
        let temp: Vec<u16> = bytes_encode(12, e[i].coeffs.iter().map(|x| x.to_int()).collect());
        pk_pke.extend(temp);
    }

    (ek_pke, pk_pke)
}

pub fn kpke_enc(ek_pke: Vec<u16>, m: Vec<u16>, r: Vec<u8>, du: usize, di: usize, eta: usize) -> (Vec<u16>, Vec<u16>) {
    let k = (ek_pke.len() - 32) / 384;
    let mut n = 0;
    let mut t: Vec<Polynomial> = Vec::new();

    for i in 0..k {
        let temp: Polynomial = Polynomial::new(bytes_decode(12, ek_pke.clone()[i * 32..(i + 1) * 32].to_vec()).iter().map(|x| FF(*x)).collect());
        t.push(temp);
    }

    let rho: Vec<u8> = ek_pke.clone()[384 * k..384 * k + 32].iter().map(|x| *x as u8).collect();

    let mut a = matrix::Matrix::zero_matrix(k, k);
    let mut y = vec![Polynomial::zero_polynomial(); k];
    let mut e1 = vec![Polynomial::zero_polynomial(); k];
    let mut u = vec![Polynomial::zero_polynomial(); k];

    for i in 0..k {
        for j in 0..k {
            a.matrix[i][j] = sample_ntt(rho.clone(), j as u8, i as u8);
        }
    }

    for i in 0..k {
        y[i] = Polynomial::new(sample_poly_cbd(prf(eta, r.clone(), n), eta)).ntt();
        n += 1;
    }

    for i in 0..k {
        e1[i] = Polynomial::new(sample_poly_cbd(prf(eta, r.clone(), n), eta)).ntt();
        n += 1;
    }

    let e2 = Polynomial::new(sample_poly_cbd(prf(eta, r.clone(), n), eta));

    for i in 0..k {
        y[i] = y[i].clone().ntt();
    }
    let ay = matrix::mul(&a, y.clone());
    for i in 0..k {
        u[i] = ay[i].clone().intt() + e1[i].clone();
    }

    let nuy = Polynomial::new(decompress(bytes_decode(1, m), 1).iter().map(|x| FF(*x)).collect());
    let v = vec_mul(t.clone(), y.clone()).intt() + e2.clone() + nuy.clone();
    let mut c1: Vec<u16> = Vec::new();

    for i in 0..k {
        let temp: Vec<u16> = bytes_encode(du, compress(u[i].coeffs.iter().map(|x| x.to_int()).collect(), du as u8));
        c1.extend(temp);
    }

    let c2 = bytes_encode(di, compress(v.coeffs.iter().map(|x| x.to_int()).collect(), di as u8));
    (c1, c2)
}

