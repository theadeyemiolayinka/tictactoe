use codes::ResultCode;
use serde::{Deserialize, Serialize};

use crate::{Failure, APP_NAME, CONFIG_NAME};

pub mod codes;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub user: Option<String>,
}

impl AppConfig {
    pub fn update(&self) -> Result<(), Failure> {
        match confy::store(APP_NAME, CONFIG_NAME, self) {
            Ok(_) => Ok(()),
            Err(e) => Err(Failure {
                message: e.to_string(),
                trace: "".to_string(),
                code: ResultCode::ConfigUpdateFailed,
            }),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            user: None,
        }
    }
}
