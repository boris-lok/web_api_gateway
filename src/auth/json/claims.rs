use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: u8,
}

impl Claims {
    pub fn new(sub: String, exp: usize, role: u8) -> Self {
        Self { sub, exp, role }
    }
}