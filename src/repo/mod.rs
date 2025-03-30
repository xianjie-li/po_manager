use std::path::Path;

use db::DB;

use crate::entity::{
    employee::EntityEmployee, employee_change::EntityEmployeeChange, project::EntityProject,
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
