use super::super::todo_item::todo_item::TodoItemComponent;
use super::api::ApiResponse;
use super::api::{RequestHelper, TodoList};
use super::delete_todo_list::DeleteTodoListComponent;
use crate::components::todo_item::insert_todo_item::InsertTodoItemComponent;
use yew::{
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink,
};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

pub struct TodoListComponent {
    api: Fetch<Request<ApiResponse>, ApiResponse>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    GetApi,
}

impl Component for TodoListComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoListComponent {
            api: Default::default(),
            fetch_task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::SetApiFetchState(fetch_state) => {
                self.api.apply(fetch_state);
                true
            }
            Msg::GetApi => {
                ConsoleService::log("getApi");
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                ConsoleService::log("fetch");
                let request = RequestHelper::get();
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse, anyhow::Error>>>| {
                        let Json(data) = res.into_body();
                        Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                    },
                );
                ConsoleService::log("go fetch");
                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
                ConsoleService::log("done");

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        match self.api.as_ref().state() {
            yewtil::fetch::FetchState::NotFetching(_) => {
                html! {
                    <button onclick=self.link.callback(|_| Msg::GetApi)> {"Get lists"} </button>
                }
            }
            yewtil::fetch::FetchState::Fetching(_) => {
                html! {
                    <p> { "Loading"} </p>
                }
            }
            yewtil::fetch::FetchState::Fetched(response) => {
                response.body.iter().map(|todo_list: &TodoList| {
                    html! {
                        <div>
                            <h4><b>{&todo_list.title} </b> <DeleteTodoListComponent todo_list=todo_list/></h4>
                            <div>
                                <TodoItemComponent todo_list=todo_list/>
                                <InsertTodoItemComponent todo_list=todo_list.id/>
                            </div>
                        </div>
                    }
                }).collect()
            }
            yewtil::fetch::FetchState::Failed(_, _) => {html!{<h1>{"ERROR"}</h1>}}
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.update(Msg::GetApi);
        }
    }
}
