use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2
};

pub fn verify(hash: String, pass: String) -> bool {
    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(&hash).unwrap();
    argon2.verify_password(pass.as_bytes(), &parsed_hash).is_ok()
}

pub fn hash(pass: String, salt: &SaltString) -> Result<String, String> {
    let argon2 = Argon2::default();

    match argon2.hash_password(pass.as_bytes(), salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err("Error hashing password".to_string())
    }
}