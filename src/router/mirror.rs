use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};

pub async fn mirror(
    Path(id): Path<String>,
    Query(query): Query<i32>,
    Json(json): Json<RequestJson>,
) -> Json<ResponseData> {
    let response = ResponseData {
        json,
        path: id,
        query,
    };

    dbg!(&response);

    Json(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub json: RequestJson,
    pub path: String,
    pub query: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestJson {
    pub username: String,
    pub password: String,
    pub favorite_number: i32,
}
