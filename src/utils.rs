pub mod password {
    use anyhow::Result;

    use argon2::{
        Argon2,
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core},
    };

    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut rand_core::OsRng);

        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
