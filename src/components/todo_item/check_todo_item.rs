use super::api::{CheckTodoItem, RequestHelper, TodoItem};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink,
};
use yewtil::fetch::{Fetch, FetchAction};

use super::api::ApiResponse;

pub struct CheckTodoItemComponent {
    api: Fetch<Request<ApiResponse<TodoItem>>, ApiResponse<TodoItem>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_item: TodoItem,
}
pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<TodoItem>>),
    PatchApi,
}

impl Component for CheckTodoItemComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CheckTodoItemComponent {
            api: Default::default(),
            fetch_task: None,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::SetApiFetchState(fetch_state) => {
                self.api.apply(fetch_state);
                true
            }
            Msg::PatchApi => {
                ConsoleService::log("getApi");
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                ConsoleService::log("fetch");
                let body = CheckTodoItem {
                    id: self.props.todo_item.id,
                    checked: !self.props.todo_item.done,
                    todo_list_id: self.props.todo_item.todo_list_id,
                }
                .clone();
                ConsoleService::log(&format!("{:?}", body));
                let request = RequestHelper::patch(&body);
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
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        self.props = _props;
        true
    }

    fn view(&self) -> yew::Html {
        html! {
            <>
                <input type="checkbox" checked=self.props.todo_item.done, onclick=self.link.callback(|_| Msg::PatchApi)/>
                {match self.api.as_ref().state() {
                    yewtil::fetch::FetchState::NotFetching(_) => {
                        html!{}
                    }
                    yewtil::fetch::FetchState::Fetching(_) => {
                        html! {

                        }
                    }
                    yewtil::fetch::FetchState::Fetched(response) => {

                            html! {

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
