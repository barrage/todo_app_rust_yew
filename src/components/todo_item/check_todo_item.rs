use super::api::{CheckTodoItem, RequestHelper, TodoItem};
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

pub struct CheckTodoItemComponent {
    api: Fetch<Request<ApiResponse<TodoItem>>, ApiResponse<TodoItem>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_item: TodoItem,
    pub refresh: Callback<super::todo_item::Msg>,
}
pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<TodoItem>>),
    PatchApi,
    GetApi,
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
            Msg::GetApi => {
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));

                let request = RequestHelper::get_item(
                    self.props.todo_item.todo_list_id,
                    self.props.todo_item.id,
                );
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<TodoItem>, anyhow::Error>>>| {
                        let Json(data) = res.into_body();

                        Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                    },
                );

                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);

                true
            }
            Msg::PatchApi => {
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));

                let body = CheckTodoItem {
                    id: self.props.todo_item.id,
                    checked: !self.props.todo_item.done,
                    todo_list_id: self.props.todo_item.todo_list_id,
                }
                .clone();

                let request = RequestHelper::patch(&body);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<TodoItem>, anyhow::Error>>>| {
                        let Json(data) = res.into_body();

                        Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                    },
                );

                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);

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
                {
                    match self.api.as_ref().state() {
                        yewtil::fetch::FetchState::NotFetching(_) => {
                            html!{}
                        }
                        yewtil::fetch::FetchState::Fetching(_) => {
                            html!{}
                        }
                        yewtil::fetch::FetchState::Fetched(response) => {
                            html!{
                                <div class="form-check form-switch">
                                    <input class="form-check-input" type="checkbox" checked=response.body[0].done onclick=self.link.callback(|_| Msg::PatchApi)/>
                                </div>
                            }
                        }
                        yewtil::fetch::FetchState::Failed(_, _) => {
                            html!{}
                        }
                    }
                }
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.update(Msg::GetApi);
        }
    }

    fn destroy(&mut self) {}
}
