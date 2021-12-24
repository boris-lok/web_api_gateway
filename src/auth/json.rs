use serde::Deserialize;

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}