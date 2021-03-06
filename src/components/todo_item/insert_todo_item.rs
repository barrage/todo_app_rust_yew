use super::api::{InputTodoItem, RequestHelper, TodoItem};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, InputData,
};
use yewtil::fetch::{Fetch, FetchAction};

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
    pub todo_list: i32,
    pub refresh: Callback<super::todo_item::Msg>,
}
pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<TodoItem>>),
    PostApi,
    Inserted(super::todo_item::Msg),
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
                match fetch_state {
                    FetchAction::NotFetching => {}
                    FetchAction::Fetching => {}
                    FetchAction::Fetched(_) => {
                        self.update(Msg::Inserted(super::todo_item::Msg::GetApi));
                    }
                    FetchAction::Failed(_) => {}
                };
                self.api.apply(fetch_state);
                true
            }
            Msg::PostApi => {
                if self.insert_title.is_empty() {
                    Msg::SetApiFetchState(FetchAction::NotFetching);
                    return true;
                }
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                let body = InputTodoItem {
                    title: self.insert_title.clone(),
                    todo_list_id: self.props.todo_list,
                }
                .clone();
                let request = RequestHelper::post(&body);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<TodoItem>, anyhow::Error>>>| {
                        let Json(data) = res.into_body();
                        match data {
                            Ok(d) => Msg::SetApiFetchState(FetchAction::Fetched(d)),
                            Err(_) => Msg::SetApiFetchState(FetchAction::NotFetching),
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
                true
            }
            Msg::UpdateInsertTitle(new_title) => {
                self.insert_title = new_title;
                true
            }
            Msg::Inserted(m) => {
                self.props.refresh.emit(m);
                false
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
                <div class="input-group mb-3">
                    <input type="text" class="form-control" placeholder="Title" oninput=self.link.callback(|e : InputData| Msg::UpdateInsertTitle(e.value)) value=self.insert_title.clone()/>
                    <div class="input-group-append">
                        <button class="btn btn-outline-secondary" style="color:black; background-color:#def2f1" type="button" onclick=self.link.callback(|_| Msg::PostApi)>
                        { "Add new item" }
                        </button>
                    </div>
                </div>
                {
                    match self.api.as_ref().state() {
                        yewtil::fetch::FetchState::NotFetching(_) => {
                            html!{}
                        }
                        yewtil::fetch::FetchState::Fetching(_) => {
                            html! {}
                        }
                        yewtil::fetch::FetchState::Fetched(response) => {
                            html! {
                                <div class="alert alert-success" role="alert">
                                <strong> {"Inserted:  "} </strong> {&response.body[0].title}
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

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}
