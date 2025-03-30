use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    entity::employee::{DTOEmployeeCreate, DTOEmployeeParam, EmployeeStatus, EntityEmployee},
    repo::db::{DB, DBType},
    result::response::{AppResponse, AppResult},
};

pub async fn create(
    Extension(db): Extension<DBType<EntityEmployee>>,
    Json(employee): Json<DTOEmployeeCreate>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let new_employee = EntityEmployee {
        id: Uuid::new_v4().to_string(),
        name: employee.name,
        status: employee.status.unwrap_or(EmployeeStatus::Working),
    };

    employee_db.insert(0, new_employee);

    EntityEmployee::store(&employee_db)?;

    AppResponse::ok(employee_db.first())
}

pub async fn list(
    Extension(db): Extension<DBType<EntityEmployee>>,
    Query(employee): Query<DTOEmployeeParam>,
) -> AppResult {
    let employee_db = db.lock().unwrap();

    let res: Vec<&EntityEmployee> = employee_db
        .iter()
        .filter(|p| {
            let mut pass = true;

            if let Some(cur) = &employee.id {
                if p.id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &employee.name {
                if !p.name.contains(cur) {
                    pass = false;
                }
            }

            if let Some(cur) = &employee.status {
                if p.status != *cur {
                    pass = false;
                }
            }

            pass
        })
        .collect();

    AppResponse::ok(res)
}

pub async fn get(
    Extension(db): Extension<DBType<EntityEmployee>>,
    Path(id): Path<String>,
) -> AppResult {
    let employee_db = db.lock().unwrap();

    let cur = employee_db.iter().find(|p| p.id == id);

    AppResponse::ok(cur)
}

pub async fn delete(
    Extension(db): Extension<DBType<EntityEmployee>>,
    Path(id): Path<String>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let ind = employee_db.iter().position(|p| p.id == id);

    if ind.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let ind = ind.unwrap();

    EntityEmployee::store(&employee_db)?;

    AppResponse::ok(employee_db.remove(ind))
}

pub async fn update(
    Extension(db): Extension<DBType<EntityEmployee>>,
    Path(id): Path<String>,
    Json(employee): Json<DTOEmployeeParam>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let cur = employee_db.iter_mut().find(|p| p.id == id);

    if cur.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let cur: &mut EntityEmployee = cur.unwrap();

    if let Some(name) = employee.name {
        cur.name = name;
    }
    if let Some(code) = employee.status {
        cur.status = code;
    }

    let res = AppResponse::ok(cur);

    EntityEmployee::store(&employee_db)?;

    res
}
