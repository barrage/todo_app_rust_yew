use super::api::{RequestHelper, TodoItem};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink,
};
use yewtil::fetch::{Fetch, FetchAction};

use super::api::ApiResponse;

pub struct DeleteTodoItemComponent {
    api: Fetch<Request<ApiResponse<String>>, ApiResponse<String>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_item: TodoItem,
}
pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<String>>),
    DeleteApi,
}

impl Component for DeleteTodoItemComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DeleteTodoItemComponent {
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

                let request = RequestHelper::delete(&self.props.todo_item);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<String>, anyhow::Error>>>| {
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
                        { "x" }
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
