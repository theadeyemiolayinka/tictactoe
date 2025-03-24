use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ResultCode {
    Success = 0,
    SuccessAnalyticsFailed = 1,
    InvalidArgs = 15,
    PCNameNotSet = 19,
    AnalyticsFetchFailed = 26,
    ConfigUpdateFailed = 39,
    PathError = 66,
    DbError = 68,
    SerializationError = 72,
    EnvError = 77,
    CryptoError = 78,
}

impl ResultCode {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}