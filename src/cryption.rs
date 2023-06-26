use anyhow::anyhow;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand::Rng;

pub fn new(input: &[u8]) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    match Pbkdf2.hash_password(input, &salt) {
        Ok(str) => Ok(str.to_string()),
        Err(err) => Err(anyhow!("{:?}", err)),
    }
}
pub fn verify(input: &[u8], hash: String) -> anyhow::Result<bool> {
    let parsed_hash = match PasswordHash::new(&hash) {
        Ok(v) => v,
        Err(err) => return Err(anyhow!("{:?}", err)),
    };
    match Pbkdf2.verify_password(input, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn get_id(index: usize) -> Vec<char> {
    let mut s = vec![];
    for _ in 0..index {
        s.push(rand::thread_rng().gen_range(48..126) as u8 as char);
    }
    s
}
