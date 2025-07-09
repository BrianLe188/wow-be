use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: String) -> Result<String, String> {
    hash(password, DEFAULT_COST).map_err(|err| err.to_string())
}

pub fn verify_password(password: String, hashed: &str) -> Result<bool, String> {
    verify(password, hashed).map_err(|err| err.to_string())
}
