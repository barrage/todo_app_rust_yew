
use std::{str};

use serde::{Deserialize, Serialize};
use yew::{format::{Json, Nothing}, services::fetch::Request};
use yewtil::fetch::{FetchRequest, MethodBody};

use crate::NewTodoList;
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoList{
    pub id: i32,
    pub title: String,
    //pub items: Vec<TodoItem>
}
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputTodoList{
    pub title: String,

}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse {
    pub code: i32,
    pub message: String,
    pub body: Vec<TodoList>,
}

pub struct RequestHelper {}

static BASE_URL : &str = "http://localhost:8081/todo_lists";

impl RequestHelper {
    
    pub fn get() -> Request<Nothing> {
        Request::get(BASE_URL).body(Nothing).expect("Cannot build url")
    }

    pub fn post(body: &InputTodoList) -> Request<Json<&InputTodoList>> {
        Request::post("http://localhost:8081/todo_lists")
        .header("Content-Type", "application/json")
        .body(Json(body))
        .unwrap()
    }    
}