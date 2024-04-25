use anyhow::{anyhow, Error};

pub fn env_var_error_to_anyhow_error(error: std::env::VarError) -> Error {
    match error {
        std::env::VarError::NotPresent => anyhow!("ENV_VAR_ERROR Value not present!"),
        std::env::VarError::NotUnicode(str) => {
            anyhow!("ENV_VAR_ERROR Value not unicode! {:?}", str)
        }
    }
}
