use crate::db;
use crate::errors::{AppError, AppErrorType};
use crate::models::{CreateListList, Health};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(Health::up())
}

pub async fn get_lists(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError {
        message: None,
        cause: Some(err.to_string()),
        error_type: AppErrorType::DbError,
    })?;
    let result = db::get_lists(&client).await;
    result.map(|lists| HttpResponse::Ok().json(lists))
}

pub async fn get_list_items(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError {
        message: None,
        cause: Some(err.to_string()),
        error_type: AppErrorType::DbError,
    })?;
    let result = db::get_list_items(&client, path.0).await;
    result.map(|items| HttpResponse::Ok().json(items))
}

pub async fn create_list(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateListList>,
) -> impl Responder {
    let client: Client = db_pool.get().await.map_err(|err| AppError {
        message: None,
        cause: Some(err.to_string()),
        error_type: AppErrorType::DbError,
    })?;
    let result = db::create_list(&client, json.title.clone()).await;
    result.map(|list| HttpResponse::Ok().json(list))
}
