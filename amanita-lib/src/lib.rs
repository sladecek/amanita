use dusk_bls12_381::{G1Affine, G2Affine, Gt, BlsScalar, G1Projective, pairing};
use ff::Field;
use group::Group;
use rand_core::RngCore;
use sha2::{Sha256, Digest};
use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    consts::{U16, U32}, generic_array::GenericArray,
};

pub fn setup_algorithm(mut rng: impl RngCore) -> (MasterSecretKey, MasterPublicKey) {
    let alpha = BlsScalar::random(&mut rng);
    let beta = BlsScalar::random(&mut rng);
    let u0 = G1Affine::from(G1Affine::generator() * alpha);
    let u1 = G2Affine::from(G2Affine::generator() * alpha);
    let v0 = G1Affine::from(G1Affine::generator() * beta);
    let v1 = G2Affine::from(G2Affine::generator() * beta);
    let w0 = G1Affine::from(G1Projective::random(&mut rng));
    let w = pairing(&w0, &G2Affine::generator());
    let msk = MasterSecretKey{u0, v0, w0};
    let mpk = MasterPublicKey{u1, v1, w};
    (msk, mpk)
}

#[derive(Debug)]
pub struct MasterSecretKey {
    u0: G1Affine,
    v0: G1Affine,
    w0: G1Affine
}

impl MasterSecretKey {
    // read - save from a file
    pub fn key_generation_algorithm(self, mut rng: impl RngCore, id: &NetworkIdentifier) -> DecryptionKey {
        let gamma = BlsScalar::random(&mut rng);
        let a0 = G1Affine::from(self.w0 + (self.u0 * id.id + self.v0) * gamma);
        let b0 = G1Affine::from(G1Affine::generator()*gamma);
        DecryptionKey{a0, b0}
    }
}

#[derive(Debug)]
pub struct MasterPublicKey {
    u1: G2Affine,
    v1: G2Affine,
    w: Gt
}
 

impl MasterPublicKey {
    // read and write to file

    pub fn encryption_algorithm(self, mut rng: impl RngCore, id: &NetworkIdentifier, uid: &UserIdentifier) -> Cryptogram {
        let ro = BlsScalar::random(&mut rng);
        let x1 = G2Affine::from(G2Affine::generator() * ro);
        let y1 = G2Affine::from((self.u1 * id.id + self.v1) * ro);
        let z = self.w * ro;
        
        let k = hash_key(&x1, &y1, &z);

        let cipher = Aes256::new(&k.into());
        let mut block = uid.id.clone();
        cipher.encrypt_block(&mut block);
        let c = block.clone();
        Cryptogram {x1: x1.to_compressed(), y1: y1.to_compressed(), c}
    }       
   
}

fn hash_key(x1: &G2Affine, y1: &G2Affine, z: &Gt) -> GenericArray<u8, U32> {
    let bytes = rkyv::to_bytes::<_, 256>(&(*x1, *y1, *z)).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize()
}


#[derive(Debug)]
struct DecryptionKey {
    a0: G1Affine,
    b0: G1Affine
}

impl DecryptionKey {
    pub fn decryption_algorithm(self, c: &Cryptogram) -> UserIdentifier {
        let x1 = G2Affine::from_compressed(&c.x1).unwrap();
        let y1 = G2Affine::from_compressed(&c.y1).unwrap();
        let z =  pairing(&self.a0, &x1) - pairing(&self.b0, &y1);      
        let k = hash_key(&x1, &y1, &z);
        let cipher = Aes256::new(&k.into());
        let mut block = c.c.clone();
        cipher.decrypt_block(&mut block);
        let id = block.clone();      
        UserIdentifier{id}  
    }
}


#[derive(Debug)]
pub struct NetworkIdentifier {
    pub id: BlsScalar
}


impl NetworkIdentifier {
    pub fn new(id: u64) -> NetworkIdentifier {
        NetworkIdentifier{id :  id.into()}
    }
}


#[derive(Debug)]
pub struct UserIdentifier {
    pub id: GenericArray<u8, U16>
}

impl UserIdentifier {
    pub fn new_random(rng: &mut impl RngCore) -> UserIdentifier {
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);
        UserIdentifier{ id: bytes.into()}
    }
}

#[derive(Debug)]
struct Cryptogram {
    x1: [u8;96],
    y1: [u8;96],
    c: GenericArray<u8, U16>
}

pub fn compact_test(mut rng: impl RngCore) -> bool {
    let (msk, mpk) = setup_algorithm(&mut rng);
    let uid = UserIdentifier::new_random(&mut rng);
    let id = NetworkIdentifier::new(1);
    let cryptogram = mpk.encryption_algorithm(&mut rng, &id, &uid);
    let decryption_key = msk.key_generation_algorithm(rng, &id);
    let uid2 = decryption_key.decryption_algorithm(&cryptogram);
    uid.id == uid2.id
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{RngCore, Error, impls::fill_bytes_via_next};

    struct FakeRng(u64);

        impl RngCore for FakeRng {
            fn next_u32(&mut self) -> u32 {
                self.next_u64() as u32
            }
        
            fn next_u64(&mut self) -> u64 {
                self.0 += 1;
                self.0
            }
        
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                fill_bytes_via_next(self, dest)
            }
        
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
                Ok(self.fill_bytes(dest))
            }
    }

    #[test]
    fn it_works() {
        let mut rng = FakeRng(27u64);
        let (msk, mpk) = setup_algorithm(&mut rng);
        let uid = UserIdentifier::new_random(&mut rng);
        let id = NetworkIdentifier::new(1);
        let cryptogram = mpk.encryption_algorithm(&mut rng, &id, &uid);
        let decryption_key = msk.key_generation_algorithm(rng, &id);
        let uid2 = decryption_key.decryption_algorithm(&cryptogram);
        println!("cryptogram: {:?}", cryptogram);
        assert_eq!(uid.id, uid2.id);
    }

    #[test]
    fn try_compact_test() {
        assert!(compact_test(FakeRng(0u64)));
    }

}