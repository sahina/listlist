use crate::errors::{AppError, AppErrorType};
use crate::models::{ListItem, ListList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_lists(client: &Client) -> Result<Vec<ListList>, AppError> {
    let query = client
        .prepare("select * from list_list order by id desc limit 10")
        .await
        .map_err(AppError::db_error)?;
    let lists = client
        .query(&query, &[])
        .await
        .expect("Error getting list lists")
        .iter()
        .map(|row| ListList::from_row_ref(row).unwrap())
        .collect::<Vec<ListList>>();

    Ok(lists)
}

pub async fn get_list_items(client: &Client, list_id: i32) -> Result<Vec<ListItem>, AppError> {
    let query = client
        .prepare("select * from list_item where list_id = $1 order by id limit 10")
        .await
        .map_err(AppError::db_error)?;
    let items = client
        .query(&query, &[&list_id])
        .await
        .expect("Error getting list items")
        .iter()
        .map(|row| ListItem::from_row_ref(row).unwrap())
        .collect::<Vec<ListItem>>();

    Ok(items)
}

pub async fn create_list(client: &Client, title: String) -> Result<ListList, AppError> {
    let query = client
        .prepare("insert into list_list (title) values ($1) returning id, title")
        .await
        .map_err(AppError::db_error)?;
    client
        .query(&query, &[&title])
        .await
        .expect("Error creating list")
        .iter()
        .map(|row| ListList::from_row_ref(row).unwrap())
        .collect::<Vec<ListList>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating list list".to_string()),
            cause: Some("Unknown error".to_string()),
            error_type: AppErrorType::DbError,
        })
}
