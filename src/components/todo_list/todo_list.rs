use super::super::todo_item::todo_item::TodoItemComponent;
use super::api::ApiResponse;
use super::api::{RequestHelper, TodoList};
use super::delete_todo_list::DeleteTodoListComponent;
use crate::{components::todo_list::insert_todo_list::InsertTodoListComponent, routes::AppRoute};
use yew::{
    format::{Json, Nothing},
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink,
};
use yew_router::{components::RouterAnchor, prelude::RouteAgent, agent::RouteRequest::ChangeRoute};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;

pub struct TodoListComponent {
    api: Fetch<Request<ApiResponse>, ApiResponse>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    GetApi,
    Ignore,
    //ChangeRoute(AppRoute)
}
#[derive(Properties, Clone)]
pub struct Props {
    
}

impl Component for TodoListComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoListComponent {
            api: Default::default(),
            fetch_task: None,
            
            props: _props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
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
                        ConsoleService::log(&format!("{:?}", data));
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
            Msg::Ignore => {true}
            /*Msg::ChangeRoute(route) => {
                &self.router_agent.send(ChangeRoute(route.into()));
                true
            }*/
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
                <div class="container" style="height: 100px"> </div>
                <InsertTodoListComponent refresh= refresh_cb.clone()/>

                {match self.api.as_ref().state() {
                    yewtil::fetch::FetchState::NotFetching(_) => {
                        html! {
                            
                        }
                    }
                    yewtil::fetch::FetchState::Fetching(_) => {
                        html! {
                            <p> { "Loading"} </p>
                        }
                    }
                    yewtil::fetch::FetchState::Fetched(response) => {
                        let mut body = response.body.clone();
                        body.sort_by(|a,b| b.id.cmp(&a.id));
                        body.iter().map(|todo_list: &TodoList| {
                            
                            html! {
                                <div class="card m-1" style="background-color:#2b7a78; text-color: white ">
                                    
                                    <div class="card-body d-flex justify-content-between align-items-center" >
                                        
                                        <RouterAnchor<AppRoute> route=AppRoute::TodoList(todo_list.id)>
                                             <span style="color: white"><b>{&todo_list.title} </b></span>
                                        </RouterAnchor<AppRoute>>
                                         
                                            <DeleteTodoListComponent todo_list=todo_list.clone() refresh=refresh_cb.clone()/>
                                        </div>
                                        
                                   
                                    /*<div>
                                        <TodoItemComponent todo_list=todo_list.clone()/>

                                    </div>*/
                                </div>
                            }
                        }).collect()
                    }
                    yewtil::fetch::FetchState::Failed(_, _) => {html!{<h1>{"ERROR"}</h1>}}
                }}
            </>
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.update(Msg::GetApi);
        }
    }
}
