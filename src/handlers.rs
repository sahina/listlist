use crate::db;
use crate::models::{CreateListList, Health};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(Health::up())
}

pub async fn get_lists(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connectiong to db");
    let result = db::get_lists(&client).await;
    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_list_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connectiong to db");
    let result = db::get_list_items(&client, path.0).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_list(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateListList>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connectiong to db");
    let result = db::create_list(&client, json.title.clone()).await;
    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
