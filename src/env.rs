use std::env::{var, VarError};

use dotenv::dotenv;

pub fn load_env() {
    if let Err(error) = dotenv() {
        error!("Error loading variables from .env: {}", error);
    }
}

pub fn get(key: &str) -> Result<String, VarError> {
    var(key)
}
