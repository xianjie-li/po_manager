use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::serde_custom::date_format::{date_format, date_format_option};

pub mod employee;
pub mod employee_change;
pub mod project;

/// 特殊出勤记录, 记录非正常出勤, 比如加班/调休/请假
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityAttendance {
    pub id: String,
    /// 开始时间
    #[serde(with = "date_format")]
    pub start_time: NaiveDate,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NaiveDate>,
    /// 人员id
    pub employee_id: String,
    /// 项目id
    pub project_id: String,
}

/// 特殊出勤记录创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOAttendance {
    pub id: Option<String>,
    /// 开始时间
    #[serde(with = "date_format")]
    pub start_time: NaiveDate,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub end_time: Option<NaiveDate>,
    /// 人员id
    pub employee_id: String,
    /// 项目id
    pub project_id: String,
}

/// 特殊日期, 记录周末以外的节假日, 或是不应记为节假日的周末
#[derive(Serialize, Deserialize, Debug)]
pub struct EntitySpecialDate {
    pub id: String,
    /// 日期
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    /// 考勤类型, 计入假日/从假日排除
    pub date_type: SpecialDateType,
}

/// 特殊日期创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOSpecialDate {
    pub id: Option<String>,
    /// 日期
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    /// 考勤类型, 计入假日/从假日排除
    pub date_type: SpecialDateType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpecialDateType {
    /// 包含
    Include,
    /// 不包含
    Exclude,
}
