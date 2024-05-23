use loam_sdk::soroban_sdk::{env, Bytes, BytesN, String};

pub fn hash_string(s: &String) -> BytesN<32> {
    let env = env();
    let len = s.len() as usize;
    let mut bytes = [0u8; 100];
    let bytes = &mut bytes[0..len];
    s.copy_into_slice(bytes);
    let mut b = Bytes::new(env);
    b.copy_from_slice(0, bytes);
    env.crypto().sha256(&b).into()
}

pub const MAX_BUMP: u32 = 535_679;
