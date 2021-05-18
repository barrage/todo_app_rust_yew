#![recursion_limit="512"]
use std::fmt::Display;
pub mod components;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::todo_list::todo_list::TodoListComponent;
use crate::components::todo_list::insert_todo_list::InsertTodoListComponent;
use wasm_bindgen::prelude::*;
use yew::{format::{Json, Nothing}, services::ConsoleService};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Html};

use anyhow::Error;
use serde::{Deserialize, Serialize};
pub enum Msg {
    FetchJSON,
    FetchReady(Result<ApiResponse, Error>),
    InsertList,
    UpdateListName(String),
    Delete(i32),
    Try,
}
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub code: i32,
    pub message: String,
    pub body: Vec<TodoList>,
}
impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}
#[derive(Deserialize, Debug)]
pub struct TodoList {
    id: i32,
    title: String,
    items: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTodoList {
    title: String,
}
impl Display for NewTodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}
impl Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}

#[derive(Deserialize, Debug)]
pub struct TodoItem {
    id: i32,
    title: String,
    done: bool,
    todo_list_id: i32,
}
impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}
pub struct Model {
    link: ComponentLink<Self>,
    data: Option<ApiResponse>,
    fetch: Option<FetchTask>,
    pub btn: String,
    pub create_list_title: String,
}
impl Model {
    fn view_data(&self) -> Html {
        if let Some(value) = &self.data {
            html! {
                <>
                    <ul>
                    {
                        for value.body.iter().map(|l| {
                            html! { 
                                <li> {&l.title} {"   "}  <button >
                                { "x" }
                            </button>
                                 //todo items list
                                        <ul>
                                            {for l.items.iter().map(|item| {
                                                html! {
                                                    <li> {&item.title} {" --> "}{match &item.done {
                                                        true => html! {
                                                            <input type="checkbox" value=true/>
                                                        },
                                                        false => html! {
                                                            <input type="checkbox" value=false/>
                                                        },
                                                    }}</li>
                                                }
                                            })}
                                        </ul>
                                </li>
                            }
                        })
                    }
                    </ul>
                </>
               
            }
        } else {
            html! {
                <p> {"No data yet"} </p>
            }
        }
    }
    fn fetch_json(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<ApiResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Try
                }
            },
        );

        let request = Request::get("http://localhost:8081/todo_lists")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
    fn insert_list(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<ApiResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchJSON
                } else {
                    Msg::Try
                }
            },
        );
        let body = NewTodoList{title: self.create_list_title.clone()};
        let request = Request::post("http://localhost:8081/todo_lists")
        .header("Content-Type", "application/json")
            
        .body(Json(&body))
        .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
    fn delete_list(&mut self, id: i32) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<ApiResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchJSON
                } else {
                    Msg::Try
                }
            },
        );
        
        let request = Request::delete(format!("http://localhost:8081/todo_lists/{}", id))
            
        .body(Nothing)
        .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            data: None,
            fetch: None,
            btn: String::from("Button click"),
            create_list_title: String::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchJSON => {
                let task = self.fetch_json();
                self.fetch = Some(task);
                true
            }
            Msg::FetchReady(response) => {
                self.data = response.map(|data| data).ok();
                true
            },
            Msg::UpdateListName(str) => {
                self.create_list_title = str;
                true
            }
            Msg::InsertList => {
                let title = &self.create_list_title;
                ConsoleService::log(title);
                self.fetch = Some(self.insert_list());

                true
            },
            Msg::Delete(id) => {
                self.fetch = Some(self.delete_list(id));
                true
            }

            Msg::Try => {
                
                true
            }
            
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header/>
                <div>
                    <button onclick=self.link.callback(|_| Msg::FetchJSON)>
                            { &self.btn }
                    </button>
                    {self.view_data()}
                    <textarea oninput=self.link.callback(|e: InputData| Msg::UpdateListName(e.value)) value=&self.create_list_title></textarea>
                    <button onclick=self.link.callback(|_| Msg::InsertList)>
                            { "Add new list" }
                    </button>
                    </div>
                <InsertTodoListComponent/>
                <TodoListComponent/>
                <Footer/>
            </>

        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
