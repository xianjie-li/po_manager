use std::path::Path;

use db::DB;

use crate::entity::{
    attendance::EntityAttendance, employee::EntityEmployee, employee_change::EntityEmployeeChange,
    project::EntityProject, special_date::EntitySpecialDate,
};

pub mod db;

impl DB for EntityProject {
    type Entity = EntityProject;

    fn get_path() -> impl AsRef<Path> {
        "/Users/lixianjie/project/proj/po_manager/db/project.json"
    }
}

impl DB for EntityEmployee {
    type Entity = EntityEmployee;

    fn get_path() -> impl AsRef<Path> {
        "/Users/lixianjie/project/proj/po_manager/db/employee.json"
    }
}

impl DB for EntityEmployeeChange {
    type Entity = EntityEmployeeChange;

    fn get_path() -> impl AsRef<Path> {
        "/Users/lixianjie/project/proj/po_manager/db/employee_change.json"
    }
}

impl DB for EntityAttendance {
    type Entity = EntityAttendance;

    fn get_path() -> impl AsRef<Path> {
        "/Users/lixianjie/project/proj/po_manager/db/attendance.json"
    }
}

impl DB for EntitySpecialDate {
    type Entity = EntitySpecialDate;

    fn get_path() -> impl AsRef<Path> {
        "/Users/lixianjie/project/proj/po_manager/db/special_date.json"
    }
}
