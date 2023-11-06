use std::env::{VarError, var};

use dotenv::dotenv;

pub fn load_env() {
    dotenv().expect("Error loading .env");
}

pub fn get(key: &str) -> Result<String, VarError> {
    var(key)
}
