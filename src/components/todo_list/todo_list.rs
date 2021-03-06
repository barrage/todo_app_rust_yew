
use super::api::ApiResponse;
use super::api::{RequestHelper, TodoList};
use super::delete_todo_list::DeleteTodoListComponent;
use crate::{components::todo_list::insert_todo_list::InsertTodoListComponent, routes::AppRoute};
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
use yew_router::{components::RouterAnchor};
use yewtil::fetch::{Fetch, FetchAction};

pub struct TodoListComponent {
    api: Fetch<Request<ApiResponse>, ApiResponse>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    GetApi,
    Ignore,
}
#[derive(Properties, Clone)]
pub struct Props {}

impl Component for TodoListComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoListComponent {
            api: Default::default(),
            fetch_task: None,
            props: _props,
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
                self.link
                    .send_message(Msg::SetApiFetchState(FetchAction::Fetching));

                let request = RequestHelper::get();
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse, anyhow::Error>>>| {
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
            Msg::Ignore => true, 
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        self.props = _props;
        self.update(Msg::GetApi);
        true
    }

    fn view(&self) -> yew::Html {
        let refresh_cb = self.link.callback(|_| Msg::GetApi);
        html! {
            <>
                <div class="d-flex align-items-center " style="height: 150px">
                    <h3 class="" > {"My lists"} </h3>
                </div>
                <InsertTodoListComponent refresh= refresh_cb.clone()/>

                {
                    match self.api.as_ref().state() {
                        yewtil::fetch::FetchState::NotFetching(_) => {
                            html! {}
                        }
                        yewtil::fetch::FetchState::Fetching(_) => {
                            html! {}
                        }
                        yewtil::fetch::FetchState::Fetched(response) => {
                            let mut body = response.body.clone();
                            body.sort_by(|a,b| b.id.cmp(&a.id));
                            body.iter().map(|todo_list: &TodoList| {

                                html! {
                                    <div class="card m-1" style="background-color:#2b7a78; text-color: white ">
                                        <div class="card-body d-flex justify-content-between align-items-center">
                                            <RouterAnchor<AppRoute> route=AppRoute::TodoList(todo_list.id)>
                                                <span style="color: white"><b>{&todo_list.title} </b></span>
                                            </RouterAnchor<AppRoute>>
                                            <DeleteTodoListComponent todo_list=todo_list.clone() refresh=refresh_cb.clone()/>
                                        </div>
                                    </div>
                                }
                            }).collect()
                        }
                        yewtil::fetch::FetchState::Failed(_, _) => {html!{}}
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
}
