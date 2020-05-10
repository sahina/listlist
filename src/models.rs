use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Health {
    pub status: String,
}

impl Health {
    pub fn up() -> Self {
        Health {
            status: String::from("UP"),
        }
    }
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "list_list")]
pub struct ListList {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "list_item")]
pub struct ListItem {
    pub id: i32,
    pub title: String,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct CreateListList {
    pub title: String,
}
