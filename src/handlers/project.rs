use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    entity::project::{DTOProjectCreate, DTOProjectParam, EntityProject},
    repo::db::{DB, DBType},
    result::response::{AppResponse, AppResult},
};

pub async fn create(
    Extension(db): Extension<DBType<EntityProject>>,
    Json(project): Json<DTOProjectCreate>,
) -> AppResult {
    let mut project_db = db.lock().unwrap();

    let new_project = EntityProject {
        id: Uuid::new_v4().to_string(),
        name: project.name,
        code: project.code,
        release_date: project.release_date,
        plan_delivery_date: project.plan_delivery_date,
        tech_days: project.tech_days,
        test_days: project.test_days,
        price: project.price,
        pm: project.pm,
    };

    project_db.insert(0, new_project);

    EntityProject::store(&project_db)?;

    AppResponse::ok(project_db.first())
}

pub async fn list(
    Extension(db): Extension<DBType<EntityProject>>,
    Query(project): Query<DTOProjectParam>,
) -> AppResult {
    let project_db = db.lock().unwrap();

    let res: Vec<&EntityProject> = project_db
        .iter()
        .filter(|p| {
            let mut pass = true;

            if let Some(cur) = &project.id {
                if p.id != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &project.name {
                if !p.name.contains(cur) {
                    pass = false;
                }
            }

            if let Some(cur) = &project.code {
                if !p.code.contains(cur) {
                    pass = false;
                }
            }

            if let Some(cur) = &project.release_date {
                if p.release_date != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &project.plan_delivery_date {
                if p.plan_delivery_date != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &project.tech_days {
                if p.tech_days != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &project.test_days {
                if p.test_days != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &project.price {
                if p.price != *cur {
                    pass = false;
                }
            }

            if let Some(cur) = &project.pm {
                if p.pm != *cur {
                    pass = false;
                }
            }

            pass
        })
        .collect();

    AppResponse::ok(res)
}

pub async fn get(
    Extension(db): Extension<DBType<EntityProject>>,
    Path(id): Path<String>,
) -> AppResult {
    let project_db = db.lock().unwrap();

    let cur = project_db.iter().find(|p| p.id == id);

    AppResponse::ok(cur)
}

pub async fn delete(
    Extension(db): Extension<DBType<EntityProject>>,
    Path(id): Path<String>,
) -> AppResult {
    let mut project_db = db.lock().unwrap();

    let ind = project_db.iter().position(|p| p.id == id);

    if ind.is_none() {
        return AppResponse::<()>::err("记录不存在");
    }

    let ind = ind.unwrap();

    EntityProject::store(&project_db)?;

    AppResponse::ok(project_db.remove(ind))
}

pub async fn update(
    Extension(db): Extension<DBType<EntityProject>>,
    Path(id): Path<String>,
    Json(project): Json<DTOProjectParam>,
) -> AppResult {
    let mut project_db = db.lock().unwrap();

    let cur = project_db.iter_mut().find(|p| p.id == id);

    if cur.is_none() {
        return AppResponse::<()>::err("项目不存在");
    }

    let cur: &mut EntityProject = cur.unwrap();

    if let Some(name) = project.name {
        cur.name = name;
    }
    if let Some(code) = project.code {
        cur.code = code;
    }
    if let Some(release_date) = project.release_date {
        cur.release_date = release_date;
    }
    if let Some(plan_delivery_date) = project.plan_delivery_date {
        cur.plan_delivery_date = plan_delivery_date;
    }
    if let Some(tech_days) = project.tech_days {
        cur.tech_days = tech_days;
    }
    if let Some(test_days) = project.test_days {
        cur.test_days = test_days;
    }
    if let Some(price) = project.price {
        cur.price = price;
    }
    if let Some(pm) = project.pm {
        cur.pm = pm;
    }

    let res = AppResponse::ok(cur);

    EntityProject::store(&project_db)?;

    res
}
