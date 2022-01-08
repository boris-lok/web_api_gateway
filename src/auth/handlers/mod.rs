use jsonwebtoken::{encode, EncodingKey, Header};

use crate::auth::json::claims::Claims;

pub mod v1;

fn create_token(claims: Claims, secret_key: &str) -> String {
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    );
    token.unwrap()
}

fn get_expired_seconds() -> usize {
    30 * 24 * 60 * 60
}
