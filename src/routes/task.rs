use crate::routes::DB_TASK;
use std::collections::HashMap;

use axum::{extract};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    name: String
}

#[derive(Deserialize, Debug)]
pub struct InsertPost {
    key: String,
    value: String
}

pub async fn list(data: extract::Json<Post>) -> String {
    let task_prefix = format!("{}", data.name);

    let mut data = HashMap::new();
    let mut iter = DB_TASK.iter();
    while let Some(Ok((key, value))) = iter.next() {
        let key = String::from_utf8(key.to_vec()).unwrap();
        let value = String::from_utf8(value.to_vec()).unwrap();
        if key.starts_with(&task_prefix) {
            data.insert(key, value);
        }
    }

    format!("{}", serde_json::json!({
        "code": 200,
        "message": "success",
        "data": data
    }))
}

pub async fn insert(data: extract::Json<InsertPost>) -> String {
    match DB_TASK.insert(&data.key, &*data.value) {
        Ok(_) => format!("{}", serde_json::json!({
            "code": 200,
            "message": "success"
        })),
        Err(_) => format!("{}", serde_json::json!({
            "code": 400,
            "message": "error"
        }))
    }
}

pub async fn delete(data: extract::Json<Post>) -> String {
    match DB_TASK.remove(&data.name) {
        Ok(_) => format!("{}", serde_json::json!({
            "code": 200,
            "message": "success"
        })),
        Err(_) => format!("{}", serde_json::json!({
            "code": 400,
            "message": "error"
        }))
    }
}