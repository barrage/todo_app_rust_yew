use crate::components::todo_list::api::TodoList;

use super::api::{InputTodoItem, RequestHelper, TodoItem};
use yew::{
    prelude::*,
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink, InputData,
};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use super::api::ApiResponse;

pub struct InsertTodoItemComponent {
    api: Fetch<Request<ApiResponse<TodoItem>>, ApiResponse<TodoItem>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    insert_title: String,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_list: i32
}
pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<TodoItem>>),
    PostApi,
    UpdateInsertTitle(String),
}

impl Component for InsertTodoItemComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InsertTodoItemComponent {
            api: Default::default(),
            fetch_task: None,
            link,
            insert_title: String::new(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::SetApiFetchState(fetch_state) => {
                self.api.apply(fetch_state);
                true
            }
            Msg::PostApi => {
                if self.insert_title.is_empty() {
                    Msg::SetApiFetchState(FetchAction::NotFetching);
                    return true;
                }
                ConsoleService::log("getApi");
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                ConsoleService::log("fetch");
                let body = InputTodoItem {
                    title: self.insert_title.clone(),
                    todo_list_id: self.props.todo_list,
                }
                .clone();
                let request = RequestHelper::post(&body);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<TodoItem>, anyhow::Error>>>| {

                        let Json(data) = res.into_body();
                        ConsoleService::log(&format!("{:?}", data));
                        Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                    },
                );
                ConsoleService::log("go fetch");
                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
                ConsoleService::log("done");

                true
            }
            Msg::UpdateInsertTitle(new_title) => {
                self.insert_title = new_title;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        html! {
            <>  
                <div style="margin-left: 40px">
                    <input type="text" oninput=self.link.callback(|e : InputData| Msg::UpdateInsertTitle(e.value)) value=&self.insert_title/>
                    <button onclick=self.link.callback(|_| Msg::PostApi)>
                            { "Add new item" }
                    </button>
                </div>
                {match self.api.as_ref().state() {
                    yewtil::fetch::FetchState::NotFetching(_) => {
                        html!{}
                    }
                    yewtil::fetch::FetchState::Fetching(_) => {
                        html! {
                            <p> { "Inserting"} </p>
                        }
                    }
                    yewtil::fetch::FetchState::Fetched(response) => {


                            html! {
                                <div>
                                    <h4 style="color:green"><b>{&"Inserted: "} {&response.body[0].title}</b></h4>
                                </div>
                            }

                    }
                    yewtil::fetch::FetchState::Failed(_, _) => {html!{<h1>{"ERROR"}</h1>}}
                }}

            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}