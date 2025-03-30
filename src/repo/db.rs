use std::{
    fs::{self, File, exists},
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};

use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::result::response::AppResult;

pub type DBType<T> = Arc<Mutex<Vec<T>>>;

pub trait DB {
    /// 对应的实体
    type Entity: Serialize + for<'de> Deserialize<'de>;

    fn get_path() -> impl AsRef<Path>;

    /// 初始化数据文件, 如果文件不存在则创建, 存在则进行序列化读取
    fn new() -> DBType<Self::Entity> {
        if !exists(Self::get_path()).unwrap() {
            let mut file = File::create_new(Self::get_path()).unwrap();

            file.write_all(b"[]").unwrap();

            return Arc::new(Mutex::new(Vec::new()));
        }

        let content = fs::read_to_string(Self::get_path()).unwrap();

        let d: Vec<Self::Entity> = serde_json::from_str(content.as_str()).unwrap();

        Arc::new(Mutex::new(d))
    }

    /// 将指定数据写入当前json文件中
    fn store(db: &Vec<Self::Entity>) -> AppResult {
        let content = serde_json::to_string_pretty(db)?;

        let path = Self::get_path();

        fs::write(path, content)?;

        Ok(().into_response())
    }
}
