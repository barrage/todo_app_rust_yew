use yew::{InputData, Component, ComponentLink, format::{Json, Nothing}, html, services::{ConsoleService, FetchService, fetch::{FetchTask, Request, Response}}};
use yewtil::fetch::{Fetch, FetchAction};
use yewtil::future::LinkFuture;
use super::api::{InputTodoList, RequestHelper, TodoList};

use super::api::{ApiResponse};



pub struct InsertTodoListComponent{
    api: Fetch<Request<ApiResponse>, ApiResponse>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    insert_title: String,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    PostApi,
    UpdateInsertTitle(String),
}

impl Component for InsertTodoListComponent {
    type Message = Msg;
    type Properties = ();


    fn create(_: Self::Properties, link: ComponentLink<Self> ) -> Self {
        InsertTodoListComponent {
            api : Default::default(),
            fetch_task: None,
            link,
            insert_title: String::new(),
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
                //ConsoleService::log("getApi");
                self.link.send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                //ConsoleService::log("fetch");
                let body = InputTodoList{ title: self.insert_title.clone()}.clone();
                let request = RequestHelper::post(&body);
                let callback = self.link.callback(|res : Response<Json<Result<ApiResponse, anyhow::Error>>>| {
                    let Json(data) = res.into_body();
                    Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                });
                //ConsoleService::log("go fetch");
                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);
                //ConsoleService::log("done");
                
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
        match self.api.as_ref().state() {
            yewtil::fetch::FetchState::NotFetching(_) => {
                html! {
                    <>
                        <textarea oninput=self.link.callback(|e : InputData| Msg::UpdateInsertTitle(e.value)) value=&self.insert_title></textarea>
                        <button onclick=self.link.callback(|_| Msg::PostApi)>
                                { "Add new list" }
                        </button>
                    </>
                }
            }
            yewtil::fetch::FetchState::Fetching(_) => {
                html! {
                    <p> { "Inserting"} </p>
                }
            }
            yewtil::fetch::FetchState::Fetched(response) => {
                
                
                    html! {
                        <div> 
                            <h4><b>{&"Inserted"}</b></h4>
                        </div>
                    }
                
            }
            yewtil::fetch::FetchState::Failed(_, _) => {html!{<h1>{"ERROR"}</h1>}}
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

