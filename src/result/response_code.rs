use serde::{Deserialize, Serialize};

/// 应用响应码
#[derive(Debug, Serialize, Deserialize)]
pub enum AppResponseCode {
    Ok,
    Err,
    Unauthorized,
}