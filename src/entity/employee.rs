use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum EmployeeStatus {
    /// 在项
    Working,
    /// 撤项
    Leave,
    /// 离职
    Quit,
}

pub fn get_employee_status_meaning(status: &EmployeeStatus) -> String {
    match status {
        EmployeeStatus::Working => "在职".to_string(),
        EmployeeStatus::Leave => "请假".to_string(),
        EmployeeStatus::Quit => "离职".to_string(),
    }
}

/// 员工信息
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityEmployee {
    pub id: String,
    /// 姓名
    pub name: String,
    /// 状态
    pub status: EmployeeStatus,
    /// 岗位
    pub position: String,
}

/// 员工信息
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployee {
    pub id: String,
    /// 姓名
    pub name: String,
    /// 状态
    pub status: EmployeeStatus,
    /// 状态含义
    pub status_meaning: String,
    /// 岗位
    pub position: String,
}

/// 员工信息创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployeeCreate {
    /// 姓名
    pub name: String,
    /// 状态, 默认为 Working
    #[serde(default)]
    pub status: Option<EmployeeStatus>,
    /// 岗位
    pub position: String,
}

/// 员工信息查询等参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployeeParam {
    #[serde(default)]
    pub id: Option<String>,
    /// 姓名
    #[serde(default)]
    pub name: Option<String>,
    /// 状态, 默认为 Working
    #[serde(default)]
    pub status: Option<EmployeeStatus>,
    /// 岗位
    #[serde(default)]
    pub position: Option<String>,
}
