use serde::{Deserialize, Serialize};

use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub session_id: String,
    pub exp: usize,
}
use base64::{
    engine::general_purpose,
    Engine,
};
use rand::Rng;


pub fn generate_access_token(
    user_id: Uuid,
    session_id: Uuid,
    secret: &str,
) -> Result<String> {
    let exp =
        (Utc::now() + Duration::minutes(15))
            .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        session_id:
            session_id.to_string(),
        exp,
    };

    Ok(
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(
                secret.as_bytes(),
            ),
        )?,
    )
}

pub fn verify_access_token(
    token: &str,
    secret: &str,
) -> Result<Claims> {
    let data =
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(
                secret.as_bytes(),
            ),
            &Validation::default(),
        )?;

    Ok(data.claims)
}



pub fn generate_refresh_token()
-> String {
    let bytes: [u8; 32] =
        rand::rng().random();

    general_purpose::URL_SAFE_NO_PAD
        .encode(bytes)
}

pub fn hash_refresh_token(
    token: &str,
) -> Result<String> {
    crate::auth::password::hash_password(
        token,
    )
}


pub fn verify_refresh_token(
    token: &str,
    hash: &str,
) -> Result<bool> {
    crate::auth::password::verify_password(
        token,
        hash,
    )
}

pub fn generate_token_pair(
    user_id: Uuid,
    session_id: Uuid,
    secret: &str,
) -> Result<(String, String)> {
    let access =
        generate_access_token(
            user_id,
            session_id,
            secret,
        )?;

    let refresh =
        generate_refresh_token();

    Ok((access, refresh))
}


#[test]
fn access_token_roundtrip() {
    let secret =
        "my-secret";

    let user_id =
        uuid::Uuid::new_v4();

    let session_id =
        uuid::Uuid::new_v4();

    let token =
        generate_access_token(
            user_id,
            session_id,
            secret,
        )
        .unwrap();

    let claims =
        verify_access_token(
            &token,
            secret,
        )
        .unwrap();

    assert_eq!(
        claims.sub,
        user_id.to_string(),
    );

    assert_eq!(
        claims.session_id,
        session_id.to_string(),
    );
}

#[test]
fn refresh_token_roundtrip() {
    let refresh =
        generate_refresh_token();

    let hash =
        hash_refresh_token(
            &refresh,
        )
        .unwrap();

    assert!(
        verify_refresh_token(
            &refresh,
            &hash,
        )
        .unwrap()
    );
}