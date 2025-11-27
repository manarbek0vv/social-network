use argon2::{
    Argon2, password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng
    }
};

// Password hashing
pub fn hash_password(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2.hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash {}", e))
}

// Password parsing
pub fn verify_password(password: String, hash: String) -> Result<(), String> {
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|_e| "Failed to parse hash")?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(()),
        Err(_e) => Err("Failed to verify password".to_string()),
    }
}