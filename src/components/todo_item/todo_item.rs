use super::api::ApiResponse;
use super::api::RequestHelper;
use super::api::TodoItem;
use super::check_todo_item::CheckTodoItemComponent;
use super::delete_todo_item::DeleteTodoItemComponent;
use super::insert_todo_item::InsertTodoItemComponent;
use crate::components::{todo_list::api::TodoListWithItems};
use yew::virtual_dom::vnode::VNode;
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

pub struct TodoItemComponent {
    api: Fetch<Request<ApiResponse<TodoListWithItems>>, ApiResponse<TodoListWithItems>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_list: i32,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<TodoListWithItems>>),
    GetApi,
}
impl Component for TodoItemComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoItemComponent {
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
                let request = RequestHelper::get_items(self.props.todo_list);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<TodoListWithItems>, anyhow::Error>>>| {
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
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        self.props = _props;
        true
    }

    fn view(&self) -> yew::Html {
        let callback = self.link.callback(|m: Msg| m);
        html! {
            <>
                <div class="d-flex align-items-center " style="height: 150px">
                    <h3 class="" > {" Items"} </h3>
                </div>
                <InsertTodoItemComponent todo_list=self.props.todo_list refresh=callback.clone()/>
                {
                    match self.api.as_ref().state() {
                        yewtil::fetch::FetchState::NotFetching(_) => {
                            html! {}
                        }
                        yewtil::fetch::FetchState::Fetching(_) => {
                            html! {}
                        }
                        yewtil::fetch::FetchState::Fetched(response) => {
                            let mut body = response.body[0].clone();
                            body.items.sort_by(|a,b| b.id.cmp(&a.id));
                            html! {
                                {body.items.iter().map(|todo_item: &TodoItem| {
                                    html! {
                                        <div class="card m-1" style="background-color:#2b7a78; text-color: white ">
                                            <div class="card-body container" >
                                                <div class="row align-items-center ">
                                                    <span class="col" style="color: white; text-align:start"><b>{&todo_item.title} </b></span>
                                                    <div class="col" style="text-align:center"> <CheckTodoItemComponent  todo_item=todo_item.clone() refresh=callback.clone()/> </div>
                                                    <div class="col" style="text-align:end"><DeleteTodoItemComponent todo_item=todo_item.clone() refresh=callback.clone()/> </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<VNode>()}
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
}
