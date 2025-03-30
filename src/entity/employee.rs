use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum EmployeeStatus {
    /// 在项
    Working,
    /// 撤项
    Leave,
    /// 离职
    Quit,
}

/// 员工信息
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityEmployee {
    pub id: String,
    /// 姓名
    pub name: String,
    /// 状态
    pub status: EmployeeStatus,
}

/// 员工信息创建参数
#[derive(Serialize, Deserialize, Debug)]
pub struct DTOEmployeeCreate {
    /// 姓名
    pub name: String,
    /// 状态, 默认为 Working
    #[serde(default)]
    pub status: Option<EmployeeStatus>,
}

/// 员工信息创建参数
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
}
