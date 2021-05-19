use std::str;

use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    services::fetch::Request,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub todo_list_id: i32,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputTodoItem {
    pub title: String,
    pub todo_list_id: i32,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckTodoItem {
    pub checked: bool,
    pub id: i32,
    pub todo_list_id: i32,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub body: Vec<T>,
}

pub struct RequestHelper {}

static BASE_URL: &str = "http://localhost:8081/todo_lists";

impl RequestHelper {
    pub fn get_items(list_id: i32) -> Request<Nothing> {
        Request::get(format!("{}/{}", BASE_URL, list_id))
            .body(Nothing)
            .expect("Cannot build url")
    }
    pub fn get_item(list_id: i32, item_id: i32) -> Request<Nothing> {
        Request::get(format!("{}/{}/{}", BASE_URL, list_id, item_id))
            .body(Nothing)
            .expect("Cannot build url")
    }
    pub fn post(body: &InputTodoItem) -> Request<Json<&InputTodoItem>> {
        Request::post(format!("{}/{}", BASE_URL, body.todo_list_id))
            .header("Content-Type", "application/json")
            .body(Json(body))
            .unwrap()
    }
    pub fn delete(body: &TodoItem) -> Request<Nothing> {
        Request::delete(format!("{}/{}/{}", BASE_URL, body.todo_list_id, body.id))
            .body(Nothing)
            .expect("Error deleting")
    }
    pub fn patch(body: &CheckTodoItem) -> Request<Json<&CheckTodoItem>> {
        Request::post(format!("{}/{}/{}", BASE_URL, body.todo_list_id, body.id))
            .header("Content-Type", "application/json")
            .body(Json(body))
            .unwrap()
    }
}
