use crate::serde_custom::date_format::{date_format, date_format_option};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// 项目信息
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityProject {
    pub id: String,
    /// 名称
    pub name: String,
    /// 编码
    pub code: String,
    /// 发布日期
    #[serde(with = "date_format")]
    pub release_date: NaiveDate,
    /// 计划交付日期
    #[serde(with = "date_format")]
    pub plan_delivery_date: NaiveDate,
    /// 技术人天
    pub tech_days: i32,
    /// 测试人天
    pub test_days: i32,
    /// 报价
    pub price: f64,
    /// 项目经理
    pub pm: String,
}

/// 项目信息创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOProjectCreate {
    /// 名称
    pub name: String,
    /// 编码
    pub code: String,
    /// 发布日期
    #[serde(with = "date_format")]
    pub release_date: NaiveDate,
    /// 计划交付日期
    #[serde(with = "date_format")]
    pub plan_delivery_date: NaiveDate,
    /// 技术人天
    pub tech_days: i32,
    /// 测试人天
    pub test_days: i32,
    /// 报价
    pub price: f64,
    /// 项目经理
    pub pm: String,
}

/// 项目信息查询等参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOProjectParam {
    #[serde(default)]
    pub id: Option<String>,
    /// 名称
    #[serde(default)]
    pub name_or_code: Option<String>,
    /// 项目经理
    #[serde(default)]
    pub pm: Option<String>,
    /// 项目成员
    #[serde(default)]
    pub employee: Option<String>,
    /// 项目成员
    #[serde(default)]
    pub low_days: Option<i32>,
    /// 项目成员
    #[serde(default)]
    pub low_percent: Option<f32>,
    /// 项目成员
    #[serde(default)]
    pub release_date_fuzzy: Option<String>,
    /// 项目成员
    #[serde(default)]
    pub plan_delivery_date_fuzzy: Option<String>,
    /// 报价
    #[serde(default)]
    pub price: Option<f64>,
    /// 总人天
    #[serde(default)]
    pub days: Option<i32>,
}

/// 项目信息更新
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOProjectUpdate {
    /// 名称
    #[serde(default)]
    pub name: Option<String>,
    /// 编码
    #[serde(default)]
    pub code: Option<String>,
    /// 发布日期
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub release_date: Option<NaiveDate>,
    /// 计划交付日期
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub plan_delivery_date: Option<NaiveDate>,
    /// 技术人天
    #[serde(default)]
    pub tech_days: Option<i32>,
    /// 测试人天
    #[serde(default)]
    pub test_days: Option<i32>,
    /// 报价
    #[serde(default)]
    pub price: Option<f64>,
    /// 项目经理
    #[serde(default)]
    pub pm: Option<String>,
}
