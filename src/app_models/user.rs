
use chrono::Utc;
use serde::{Deserialize, Serialize};
use bson::DateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 该 token 的签发者 这里存储为email值
    pub sub: String, // 通常用于存储用户 ID
    /// 过期时间 这里存储为时间戳
    pub exp: usize, // 过期时间（单位：秒）
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            sub: "_".to_owned(),
            exp: (Utc::now() + chrono::Duration::hours(12)).timestamp() as usize,
        }
    }
}
