use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub exp: i64,
    pub iat: i64,
}
