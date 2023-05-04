use loam_sdk::soroban_sdk::{Bytes, BytesN, Env, String};

pub fn hash_string(env: &Env, s: &String) -> BytesN<32> {
    let len = s.len() as usize;
    let mut bytes = [0u8; 100];
    let bytes = &mut bytes[0..len];
    s.copy_into_slice(bytes);
    let mut b = Bytes::new(env);
    b.copy_from_slice(0, bytes);
    env.crypto().sha256(&b)
}
