use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::serde_custom::date_format::{date_format, date_format_option};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SpecialDateType {
    /// 视为节假日
    Include,
    /// 不视为节假日
    Exclude,
}

/// 特殊日期, 记录周末以外的节假日, 或是不应记为节假日的周末
#[derive(Serialize, Deserialize, Debug)]
pub struct EntitySpecialDate {
    pub id: String,
    /// 开始时间
    #[serde(with = "date_format")]
    pub start_time: NaiveDate,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NaiveDate>,
    /// 日期类型, 计入假日/从假日排除
    pub date_type: SpecialDateType,
}

/// 特殊日期创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOSpecialDateCreate {
    /// 开始时间
    #[serde(with = "date_format")]
    pub start_time: NaiveDate,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NaiveDate>,
    /// 日期类型, 计入假日/从假日排除
    pub date_type: SpecialDateType,
}

/// 特殊日期创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOSpecialDateParam {
    #[serde(default)]
    pub id: Option<String>,
    /// 开始时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub start_time: Option<NaiveDate>,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NaiveDate>,
    /// 日期类型, 计入假日/从假日排除
    #[serde(default)]
    pub date_type: Option<SpecialDateType>,
}
