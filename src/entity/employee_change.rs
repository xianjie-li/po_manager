use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::serde_custom::date_format::{date_format, date_format_option};

/// 人员入项和离项记录
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityEmployeeChange {
    pub id: String,
    /// 人员id
    pub employee_id: String,
    /// 项目id
    pub project_id: String,
    /// 入项时间
    #[serde(with = "date_format")]
    pub in_time: NaiveDate,
    /// 离项时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub out_time: Option<NaiveDate>,
}

/// 人员入项和离项记录
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployeeChange {
    pub id: String,
    /// 人员id
    pub employee_id: String,
    pub employee_name: String,
    /// 项目id
    pub project_id: String,
    pub project_name: String,
    /// 入项时间
    #[serde(with = "date_format")]
    pub in_time: NaiveDate,
    /// 离项时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub out_time: Option<NaiveDate>,
}

/// 人员入项和离项记录创建
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployeeChangeCreate {
    /// 人员id
    pub employee_id: String,
    /// 项目id
    pub project_id: String,
    /// 入项时间
    #[serde(with = "date_format")]
    pub in_time: NaiveDate,
    /// 离项时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub out_time: Option<NaiveDate>,
}

/// 人员入项和离项记录查询等参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployeeChangeParam {
    #[serde(default)]
    pub id: Option<String>,
    /// 人员id
    #[serde(default)]
    pub employee_id: Option<String>,
    /// 项目id
    #[serde(default)]
    pub project_id: Option<String>,
    /// 入项时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub in_time: Option<NaiveDate>,
    /// 离项时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub out_time: Option<NaiveDate>,
}
