use super::api::{InputTodoList, RequestHelper, TodoList};
use yew::{
    format::{Json, Nothing},
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink, InputData,
};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

use super::api::ApiResponse;

pub struct DeleteTodoListComponent {
    api: Fetch<Request<ApiResponse>, ApiResponse>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_list: TodoList,
}
pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    DeleteApi,
}

impl Component for DeleteTodoListComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DeleteTodoListComponent {
            api: Default::default(),
            fetch_task: None,
            link,
            props: props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::SetApiFetchState(fetch_state) => {
                self.api.apply(fetch_state);
                true
            }
            Msg::DeleteApi => {
                //ConsoleService::log("getApi");
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                //ConsoleService::log("fetch");

                let request = RequestHelper::delete(&self.props.todo_list);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse, anyhow::Error>>>| {
                        let Json(data) = res.into_body();
                        Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                    },
                );
                //ConsoleService::log("go fetch");
                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
                //ConsoleService::log("done");

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
                    <button onclick=self.link.callback(|_| Msg::DeleteApi)>
                        { "Delete" }
                    </button>
                }
            }
            yewtil::fetch::FetchState::Fetching(_) => {
                html! {}
            }
            yewtil::fetch::FetchState::Fetched(response) => match response.code {
                200 => html! { <span style="color:green;">{" Deleted"} </span> },
                500 => html! { <span style="color:red;">{" Can't delete, has children"} </span> },
                _ => html! { " -> Idk"},
            },
            yewtil::fetch::FetchState::Failed(_, _) => {
                html! {<h1>{"ERROR"}</h1>}
            }
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
