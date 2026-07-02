use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use anyhow::{anyhow, Result};
pub fn hash_password(
    password: &str,
) -> Result<String> {
    let salt =
        SaltString::generate(&mut OsRng);

    let hash =
        Argon2::default()
            .hash_password(
                password.as_bytes(),
                &salt,
            )
            .map_err(|e| anyhow!(e.to_string()))?
            .to_string();

    Ok(hash)
}

pub fn verify_password(
    password: &str,
    hash: &str,
) -> Result<bool> {
    let parsed =
    PasswordHash::new(hash)
        .map_err(|e| anyhow!(e.to_string()))?;

    Ok(
        Argon2::default()
            .verify_password(
                password.as_bytes(),
                &parsed,
            )
            .is_ok(),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_password() {
        let password =
            "secret123";

        let hash =
            hash_password(password)
                .unwrap();

        assert!(
            verify_password(
                password,
                &hash
            )
            .unwrap()
        );
    }

    #[test]
    fn wrong_password_fails() {
        let hash =
            hash_password("secret123")
                .unwrap();

        assert!(
            !verify_password(
                "wrong",
                &hash
            )
            .unwrap()
        );
    }
}