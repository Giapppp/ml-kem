use crate::kpke::*;
use crate::helper::{h, g, j, random_bytes};
use crate::constant::K;

// Algorithm 16:Uses randomness to generate an encapsulation key and a corresponding decapsulation key.
pub fn keygen_internal(d: Vec<u8>, z: Vec<u16>) -> (Vec<u16>, Vec<u16>) {
    let (ek_pke, dk_pke) = kpke_key_gen(d);
    let ek = ek_pke.clone();
    
    let mut dk = dk_pke.clone();
    dk.extend(ek.clone());
    dk.extend(h(ek.clone().iter().map(|x| *x as u8).collect()).iter().map(|x| *x as u16));
    dk.extend(z);
    assert_eq!(ek.len(), 384*K+32);
    assert_eq!(dk.len(), 768*K+96);
    (ek, dk)
}

// Algorithm 17: Uses the encapsulation key and randomness to generate a key and an associated ciphertext.
pub fn encaps_internal(ek: Vec<u16>, m: Vec<u8>) -> (Vec<u8>, Vec<u16>) {
    let mut m_ = m.clone();
    m_.append(&mut h(ek.clone().iter().map(|x| *x as u8).collect()));
    let (k, r) = g(m_.clone());
    let c = kpke_enc(ek.clone(), m.iter().map(|x| *x as u16).collect(), r);
    (k, c)
}

// Algorithm 18: Uses the decapsulation key to produce a shared secret key from a ciphertext.
pub fn decaps_internal(dk: Vec<u16>, c: Vec<u16>) -> Vec<u8> {
    let dk_pke = dk.clone()[0..384*K].to_vec();
    let ek_pke = dk.clone()[384*K..768*K+32].to_vec();
    let mut h = dk.clone()[768*K+32..768*K+64].to_vec();
    let mut z = dk.clone()[768*K+64..768*K+96].to_vec();
    let m = kpke_dec(dk_pke, c.clone());
    let mut m_ = m.clone();
    m_.append(&mut h);
    let (mut k_, r_) = g(m_.clone().iter().map(|x| *x as u8).collect());
    z.append(&mut c.clone());
    let kk = j(z.clone().iter().map(|x| *x as u8).collect());

    let c_ = kpke_enc(ek_pke.clone(), m.iter().map(|x| *x as u16).collect(), r_);
    if c != c_ {
        k_ = kk;
    } 
    k_
}

// Algorithm 19: Generates an encapsulation key and a corresponding decapsulation key.
pub fn keygen() -> (Vec<u16>, Vec<u16>) {
    let d = random_bytes(32);
    let z = random_bytes(32).iter().map(|x| *x as u16).collect();
    keygen_internal(d, z)
}

// Algorithm 20: Uses the encapsulation key to generate a shared secret key and an associated ciphertext.
pub fn encaps(ek: Vec<u16>) -> (Vec<u8>, Vec<u16>) {
    let m = random_bytes(32);
    encaps_internal(ek, m)
}

// Algorithm 21: Uses the decapsulation key to produce a shared secret key from a ciphertext.
pub fn decaps(dk: Vec<u16>, c: Vec<u16>) -> Vec<u8> {
    decaps_internal(dk, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen_encaps_decaps() {
        let (ek, dk) = keygen();
        let (k, c) = encaps(ek);
        let k_ = decaps(dk, c);
        assert_eq!(k, k_);
    }
}