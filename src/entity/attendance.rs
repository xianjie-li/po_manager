use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::serde_custom::date_format::{date_format, date_format_option};

/// 特殊考勤类型
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AttendanceType {
    /// 请假
    Leave,
    /// 调休
    CompensatoryLeave,
    /// 加班
    Overtime,
}

/// 特殊出勤记录, 记录 AttendanceType 中的非正常出勤
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
    /// 类型
    pub date_type: AttendanceType,
    /// start_time 是否表示半天
    pub start_half: bool,
    /// end_time 是否表示半天
    pub end_half: bool,
}

/// 特殊出勤记录创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOAttendanceCreate {
    /// 开始时间
    #[serde(with = "date_format")]
    pub start_time: NaiveDate,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub end_time: Option<NaiveDate>,
    /// 人员id
    pub employee_id: String,
    /// 类型
    pub date_type: AttendanceType,
    /// start_time 是否表示半天
    pub start_half: bool,
    /// end_time 是否表示半天
    pub end_half: bool,
}

/// 特殊出勤记录创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOAttendanceParam {
    #[serde(default)]
    pub id: Option<String>,
    /// 开始时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub start_time: Option<NaiveDate>,
    /// 结束时间
    #[serde(default)]
    #[serde(with = "date_format_option")]
    pub end_time: Option<NaiveDate>,
    /// 人员id
    #[serde(default)]
    pub employee_id: Option<String>,
    /// 类型
    #[serde(default)]
    pub date_type: Option<AttendanceType>,
    /// start_time 是否表示半天
    #[serde(default)]
    pub start_half: Option<bool>,
    /// end_time 是否表示半天
    #[serde(default)]
    pub end_half: Option<bool>,
}
