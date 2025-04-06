use axum::{
    Extension, Router,
    middleware::from_fn,
    routing::{get, post},
};
use entity::{
    attendance::EntityAttendance, employee::EntityEmployee, employee_change::EntityEmployeeChange,
    project::EntityProject, special_date::EntitySpecialDate,
};
use handlers::{attendance, employee, employee_change, project, special_date};
use repo::db::DB;
use result::response::text_response_process;
use tower::ServiceBuilder;

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
    let attendance = EntityAttendance::new();
    let special_date = EntitySpecialDate::new();

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
        .route("/attendance/create", post(attendance::create))
        .route("/attendance/list", get(attendance::list))
        .route("/attendance/get/{id}", get(attendance::get))
        .route("/attendance/delete/{id}", post(attendance::delete))
        .route("/attendance/update/{id}", post(attendance::update))
        .route("/special_date/create", post(special_date::create))
        .route("/special_date/list", get(special_date::list))
        .route("/special_date/get/{id}", get(special_date::get))
        .route("/special_date/delete/{id}", post(special_date::delete))
        .route("/special_date/update/{id}", post(special_date::update))
        .layer(
            ServiceBuilder::new()
                .layer(from_fn(text_response_process))
                .layer(Extension(project))
                .layer(Extension(employee))
                .layer(Extension(employee_change))
                .layer(Extension(attendance))
                .layer(Extension(special_date)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
