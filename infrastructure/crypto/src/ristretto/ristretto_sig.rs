// Copyright 2019 The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::{
    keys::PublicKey,
    ristretto::{RistrettoPublicKey, RistrettoSecretKey},
    signatures::SchnorrSignature,
};
use curve25519_dalek::scalar::Scalar;

#[allow(non_snake_case)]
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub struct RistrettoSchnorr {
    R: RistrettoPublicKey,
    s: RistrettoSecretKey,
}

impl SchnorrSignature for RistrettoSchnorr {
    type Challenge = [u8; 32];
    type Point = RistrettoPublicKey;
    type Scalar = RistrettoSecretKey;

    fn new(public_nonce: RistrettoPublicKey, signature: RistrettoSecretKey) -> Self {
        RistrettoSchnorr { R: public_nonce, s: signature }
    }

    fn sign(secret: &RistrettoSecretKey, nonce: &RistrettoSecretKey, challenge: [u8; 32]) -> RistrettoSchnorr {
        // s = r + e.k
        let e = Scalar::from_bytes_mod_order(challenge);
        let s = &nonce.0 + &(&secret.0 * &e);
        let public_nonce = RistrettoPublicKey::from_secret_key(nonce);
        RistrettoSchnorr { R: public_nonce, s: RistrettoSecretKey(s) }
    }

    fn verify(&self, public_key: &RistrettoPublicKey, public_nonce: &RistrettoPublicKey, challenge: &[u8; 32]) -> bool {
        let lhs = RistrettoPublicKey::from_secret_key(&self.s);
        let mut e = [0u8; 32];
        e.copy_from_slice(challenge);
        let e = Scalar::from_bytes_mod_order(e);
        let rhs = &public_nonce.point + &(&e * &public_key.point);
        lhs.point == rhs
    }

    fn get_signature(&self) -> &RistrettoSecretKey {
        &self.s
    }

    fn get_public_nonce(&self) -> &RistrettoPublicKey {
        &self.R
    }
}

#[cfg(test)]
mod test {
    use crate::{
        challenge::Challenge,
        common::{Blake256, ByteArray},
        keys::{PublicKey, SecretKeyFactory},
        ristretto::{RistrettoPublicKey, RistrettoSchnorr, RistrettoSecretKey},
        signatures::SchnorrSignature,
    };
    use rand;

    fn get_keypair() -> (RistrettoSecretKey, RistrettoPublicKey) {
        let mut rng = rand::OsRng::new().unwrap();
        let k = RistrettoSecretKey::random(&mut rng);
        let pk = RistrettoPublicKey::from_secret_key(&k);
        (k, pk)
    }

    #[test]
    #[allow(non_snake_case)]
    fn sign_and_verify_message() {
        let (k, P) = get_keypair();
        let (r, R) = get_keypair();
        let c = Challenge::<Blake256>::new();
        let e = c.concat(P.to_bytes()).concat(R.to_bytes()).concat(b"Small Gods");
        let e: [u8; 32] = e.into();
        let sig = RistrettoSchnorr::sign(&k.into(), &r.into(), e);
        let s_calc = sig.get_signature();
        let R_calc = sig.get_public_nonce();
        assert_eq!(R, *R_calc);
        println!("sig: {}\nmsg: {:?}", s_calc.to_hex(), e);
        assert!(sig.verify(&P, &R, &e));
    }
}
