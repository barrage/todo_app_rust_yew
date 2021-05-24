use std::str;

use serde::{Deserialize, Serialize};
use yew::{
    format::{Json, Nothing},
    services::fetch::Request,
};

use super::super::todo_item::api::TodoItem;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTodoList {
    title: String,
}

#[derive(Debug,  Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoListWithItems {
    pub id: i32,
    pub title: String,
    pub items: Vec<TodoItem>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputTodoList {
    pub title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse {
    pub code: i32,
    pub message: String,
    pub body: Vec<TodoList>,
}

pub struct RequestHelper {}

static BASE_URL: &str = "http://localhost:8081/todo_lists";

impl RequestHelper {
    pub fn get() -> Request<Nothing> {
        Request::get(BASE_URL)
            .body(Nothing)
            .expect("Cannot build url")
    }

    pub fn post(body: &InputTodoList) -> Request<Json<&InputTodoList>> {
        Request::post(BASE_URL)
            .header("Content-Type", "application/json")
            .body(Json(body))
            .unwrap()
    }
    pub fn delete(body: &TodoList) -> Request<Nothing> {
        Request::delete(format!("{}/{}", BASE_URL, body.id))
            .body(Nothing)
            .expect("Error deleting")
    }
}
