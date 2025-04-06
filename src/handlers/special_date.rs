use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    entity::special_date::{DTOSpecialDateCreate, DTOSpecialDateParam, EntitySpecialDate},
    repo::db::{DB, DBType},
    result::response::{AppResponse, AppResult},
};

pub async fn create(
    Extension(db): Extension<DBType<EntitySpecialDate>>,
    Json(special_date): Json<DTOSpecialDateCreate>,
) -> AppResult {
    let mut special_date_db = db.lock().unwrap();

    let new_employee = EntitySpecialDate {
        id: Uuid::new_v4().to_string(),
        start_time: special_date.start_time,
        end_time: special_date.end_time,
        date_type: special_date.date_type,
    };

    special_date_db.insert(0, new_employee);

    EntitySpecialDate::store(&special_date_db)?;

    AppResponse::ok(special_date_db.first())
}

pub async fn list(
    Extension(db): Extension<DBType<EntitySpecialDate>>,
    Query(special_date): Query<DTOSpecialDateParam>,
) -> AppResult {
    let special_date_db = db.lock().unwrap();

    let res: Vec<&EntitySpecialDate> = special_date_db
        .iter()
        .filter(|p| {
            let mut pass = true;

            if let Some(cur) = &special_date.id {
                if p.id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &special_date.start_time {
                if p.start_time != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &special_date.end_time {
                if p.end_time != Some(*cur) {
                    pass = false;
                }
            }

            if let Some(cur) = &special_date.date_type {
                if p.date_type != *cur {
                    pass = false;
                }
            }

            pass
        })
        .collect();

    AppResponse::ok(res)
}

pub async fn get(
    Extension(db): Extension<DBType<EntitySpecialDate>>,
    Path(id): Path<String>,
) -> AppResult {
    let special_date_db = db.lock().unwrap();

    let cur = special_date_db.iter().find(|p| p.id == id);

    AppResponse::ok(cur)
}

pub async fn delete(
    Extension(db): Extension<DBType<EntitySpecialDate>>,
    Path(id): Path<String>,
) -> AppResult {
    let mut special_date_db = db.lock().unwrap();

    let ind = special_date_db.iter().position(|p| p.id == id);

    if ind.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let ind = ind.unwrap();

    EntitySpecialDate::store(&special_date_db)?;

    AppResponse::ok(special_date_db.remove(ind))
}

pub async fn update(
    Extension(db): Extension<DBType<EntitySpecialDate>>,
    Path(id): Path<String>,
    Json(special_date): Json<DTOSpecialDateParam>,
) -> AppResult {
    let mut special_date_db = db.lock().unwrap();

    let cur = special_date_db.iter_mut().find(|p| p.id == id);

    if cur.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let cur: &mut EntitySpecialDate = cur.unwrap();

    if let Some(val) = special_date.start_time {
        cur.start_time = val;
    }

    if let Some(val) = special_date.end_time {
        cur.end_time = Some(val);
    }

    if let Some(val) = special_date.date_type {
        cur.date_type = val;
    }

    let res = AppResponse::ok(cur);

    EntitySpecialDate::store(&special_date_db)?;

    res
}
