use crate::Config;

pub fn hash_password(password: &str, config: &Config) -> Option<String> {
    let c = argon2::Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), config.secret_key.as_bytes(), &c);
    hash.ok()
}
