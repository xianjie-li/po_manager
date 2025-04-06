use std::{collections::HashMap, ops::Deref};

use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    entity::{
        employee::EntityEmployee,
        employee_change::{
            DTOEmployeeChange, DTOEmployeeChangeCreate, DTOEmployeeChangeParam,
            EntityEmployeeChange,
        },
        project::EntityProject,
    },
    repo::db::{DB, DBType},
    result::response::{AppResponse, AppResult},
};
pub async fn create(
    Extension(db): Extension<DBType<EntityEmployeeChange>>,
    Json(employee): Json<DTOEmployeeChangeCreate>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let new_employee = EntityEmployeeChange {
        id: Uuid::new_v4().to_string(),
        employee_id: employee.employee_id,
        project_id: employee.project_id,
        in_time: employee.in_time,
        out_time: employee.out_time,
    };

    employee_db.insert(0, new_employee);

    EntityEmployeeChange::store(&employee_db)?;

    AppResponse::ok(employee_db.first())
}

pub async fn list(
    Extension(employee_change_db): Extension<DBType<EntityEmployeeChange>>,
    Extension(employee_db): Extension<DBType<EntityEmployee>>,
    Extension(project_db): Extension<DBType<EntityProject>>,
    Query(employee): Query<DTOEmployeeChangeParam>,
) -> AppResult {
    let employee_change_db = employee_change_db.lock().unwrap();
    let employee_db = employee_db.lock().unwrap();
    let project_db = project_db.lock().unwrap();

    let employee_name_map = employee_db
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect::<HashMap<String, String>>();

    let project_name_map = project_db
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect::<HashMap<String, String>>();

    let res: Vec<DTOEmployeeChange> = employee_change_db
        .iter()
        .filter(|p| {
            let mut pass = true;

            if let Some(cur) = &employee.id {
                if p.id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &employee.employee_id {
                if p.employee_id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &employee.project_id {
                if p.project_id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &employee.in_time {
                if p.in_time != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &employee.out_time {
                if p.out_time != Some(*cur) {
                    pass = false;
                }
            }

            pass
        })
        .map(|p| DTOEmployeeChange {
            id: p.id.clone(),
            employee_id: p.employee_id.clone(),
            employee_name: employee_name_map
                .get(&p.employee_id)
                .unwrap_or(&String::from(""))
                .clone(),
            project_id: p.project_id.clone(),
            project_name: project_name_map
                .get(&p.project_id)
                .unwrap_or(&String::from(""))
                .clone(),
            in_time: p.in_time,
            out_time: p.out_time,
        })
        .collect();

    AppResponse::ok(res)
}

pub async fn get(
    Extension(db): Extension<DBType<EntityEmployeeChange>>,
    Path(id): Path<String>,
) -> AppResult {
    let employee_db = db.lock().unwrap();

    let cur = employee_db.iter().find(|p| p.id == id);

    AppResponse::ok(cur)
}

pub async fn delete(
    Extension(db): Extension<DBType<EntityEmployeeChange>>,
    Path(id): Path<String>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let ind = employee_db.iter().position(|p| p.id == id);

    if ind.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let ind = ind.unwrap();

    EntityEmployeeChange::store(&employee_db)?;

    AppResponse::ok(employee_db.remove(ind))
}

pub async fn update(
    Extension(db): Extension<DBType<EntityEmployeeChange>>,
    Path(id): Path<String>,
    Json(employee): Json<EntityEmployeeChange>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let ind = employee_db.iter().position(|p| p.id == id);

    if ind.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let ind = ind.unwrap();

    let res = AppResponse::ok(&employee);

    employee_db[ind] = employee;

    EntityEmployeeChange::store(&employee_db)?;

    res
}
