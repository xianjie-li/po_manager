use anyhow::anyhow;
use axum::{
    Extension, Router,
    extract::Path,
    middleware::from_fn,
    routing::{get, post},
};
use chrono::Utc;
use entity::{
    EntitySpecialDate, SpecialDateType, employee::EntityEmployee,
    employee_change::EntityEmployeeChange, project::EntityProject,
};
use handlers::{employee, employee_change, project};
use repo::db::DB;
use result::{
    response::{AppResponse, AppResult, text_response_process},
    response_code::AppResponseCode,
};
use tower::ServiceBuilder;
use uuid::Uuid;

mod entity;

mod handlers;
mod repo;
mod result;
mod serde_custom;

#[tokio::main]
async fn main() {
    let project = EntityProject::new();
    let employee = EntityEmployee::new();
    let employee_change = EntityEmployeeChange::new();

    println!("project:: {:?}", project);
    println!("employee:: {:?}", employee);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/project/create", post(project::create))
        .route("/project/list", get(project::list))
        .route("/project/get/{id}", get(project::get))
        .route("/project/delete/{id}", post(project::delete))
        .route("/project/update/{id}", post(project::update))
        .route("/employee/create", post(employee::create))
        .route("/employee/list", get(employee::list))
        .route("/employee/get/{id}", get(employee::get))
        .route("/employee/delete/{id}", post(employee::delete))
        .route("/employee/update/{id}", post(employee::update))
        .route("/employee_change/create", post(employee_change::create))
        .route("/employee_change/list", get(employee_change::list))
        .route("/employee_change/get/{id}", get(employee_change::get))
        .route(
            "/employee_change/delete/{id}",
            post(employee_change::delete),
        )
        .route(
            "/employee_change/update/{id}",
            post(employee_change::update),
        )
        .layer(
            ServiceBuilder::new()
                .layer(from_fn(text_response_process))
                .layer(Extension(project))
                .layer(Extension(employee))
                .layer(Extension(employee_change)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test_err() -> AppResult {
    try_thing()?;
    Err(anyhow!("it failed213123!").into())
}

async fn test_err2() -> AppResult {
    let data = EntitySpecialDate {
        id: Uuid::new_v4().to_string(),
        date: Utc::now().date_naive(),
        date_type: SpecialDateType::Include,
    };

    print!("{:#?} 12321", data);

    println!("{}", data.date);

    println!("{}", serde_json::to_string_pretty(&data)?);

    AppResponse::ok(data)
}

async fn test_res1(Path(id): Path<i32>) -> AppResult {
    // try_thing()?;

    // AppResponse::<()>::err("dwqdwqdwq")
    // AppResponse::ok("dwqdwqdwq")

    let res = AppResponse::new()
        .code(AppResponseCode::Unauthorized)
        .msg("hehe")
        .data("test data 1321")
        .build();

    res
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}
