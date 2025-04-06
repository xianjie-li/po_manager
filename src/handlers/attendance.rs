use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    entity::attendance::{DTOAttendanceCreate, DTOAttendanceParam, EntityAttendance},
    repo::db::{DB, DBType},
    result::response::{AppResponse, AppResult},
};

pub async fn create(
    Extension(db): Extension<DBType<EntityAttendance>>,
    Json(attendance): Json<DTOAttendanceCreate>,
) -> AppResult {
    let mut attendance_db = db.lock().unwrap();

    let new_employee = EntityAttendance {
        id: Uuid::new_v4().to_string(),
        start_time: attendance.start_time,
        end_time: attendance.end_time,
        employee_id: attendance.employee_id,
        date_type: attendance.date_type,
        start_half: attendance.start_half,
        end_half: attendance.end_half,
    };

    attendance_db.insert(0, new_employee);

    EntityAttendance::store(&attendance_db)?;

    AppResponse::ok(attendance_db.first())
}

pub async fn list(
    Extension(db): Extension<DBType<EntityAttendance>>,
    Query(attendance): Query<DTOAttendanceParam>,
) -> AppResult {
    let attendance_db = db.lock().unwrap();

    let res: Vec<&EntityAttendance> = attendance_db
        .iter()
        .filter(|p| {
            let mut pass = true;

            if let Some(cur) = &attendance.id {
                if p.id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &attendance.start_time {
                if p.start_time != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &attendance.end_time {
                if p.end_time != Some(*cur) {
                    pass = false;
                }
            }

            if let Some(cur) = &attendance.employee_id {
                if p.employee_id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &attendance.date_type {
                if p.date_type != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &attendance.start_half {
                if p.start_half != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &attendance.end_half {
                if p.end_half != *cur {
                    pass = false;
                }
            }

            pass
        })
        .collect();

    AppResponse::ok(res)
}

pub async fn get(
    Extension(db): Extension<DBType<EntityAttendance>>,
    Path(id): Path<String>,
) -> AppResult {
    let attendance_db = db.lock().unwrap();

    let cur = attendance_db.iter().find(|p| p.id == id);

    AppResponse::ok(cur)
}

pub async fn delete(
    Extension(db): Extension<DBType<EntityAttendance>>,
    Path(id): Path<String>,
) -> AppResult {
    let mut attendance_db = db.lock().unwrap();

    let ind = attendance_db.iter().position(|p| p.id == id);

    if ind.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let ind = ind.unwrap();

    EntityAttendance::store(&attendance_db)?;

    AppResponse::ok(attendance_db.remove(ind))
}

pub async fn update(
    Extension(db): Extension<DBType<EntityAttendance>>,
    Path(id): Path<String>,
    Json(employee): Json<DTOAttendanceParam>,
) -> AppResult {
    let mut employee_db = db.lock().unwrap();

    let cur = employee_db.iter_mut().find(|p| p.id == id);

    if cur.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let cur: &mut EntityAttendance = cur.unwrap();

    if let Some(val) = employee.start_time {
        cur.start_time = val;
    }

    if let Some(val) = employee.end_time {
        cur.end_time = Some(val);
    }

    if let Some(val) = employee.employee_id {
        cur.employee_id = val;
    }

    if let Some(val) = employee.date_type {
        cur.date_type = val;
    }

    if let Some(val) = employee.start_half {
        cur.start_half = val;
    }

    if let Some(val) = employee.end_half {
        cur.end_half = val;
    }

    let res = AppResponse::ok(cur);

    EntityAttendance::store(&employee_db)?;

    res
}
