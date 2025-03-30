use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    entity::employee_change::{
        DTOEmployeeChangeCreate, DTOEmployeeChangeParam, EntityEmployeeChange,
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
    Extension(db): Extension<DBType<EntityEmployeeChange>>,
    Query(employee): Query<DTOEmployeeChangeParam>,
) -> AppResult {
    let employee_db = db.lock().unwrap();

    let res: Vec<&EntityEmployeeChange> = employee_db
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
    Json(employee): Json<DTOEmployeeChangeParam>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let cur = employee_db.iter_mut().find(|p| p.id == id);

    if cur.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let cur: &mut EntityEmployeeChange = cur.unwrap();

    if let Some(val) = employee.employee_id {
        cur.employee_id = val;
    }

    if let Some(val) = employee.project_id {
        cur.project_id = val;
    }

    if let Some(val) = employee.in_time {
        cur.in_time = val;
    }

    if let Some(val) = employee.out_time {
        cur.out_time = Some(val);
    }

    let res = AppResponse::ok(cur);

    EntityEmployeeChange::store(&employee_db)?;

    res
}
