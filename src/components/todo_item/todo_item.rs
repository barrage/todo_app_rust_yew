use yew::{prelude::*, Component, ComponentLink, format::{Json, Nothing}, html, services::{ConsoleService, FetchService, fetch::{FetchTask, Request, Response}}};
use yewtil::fetch::{Fetch, FetchAction};

use super::api::TodoItem;

use super::api::ApiResponse;
use super::api::RequestHelper;
use super::super::todo_list::api::TodoList;



pub struct TodoItemComponent{
    api: Fetch<Request<ApiResponse>, ApiResponse>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props{
    pub todo_list: TodoList,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse>),
    GetApi,
}

impl Component for TodoItemComponent {
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self> ) -> Self {
        link.callback(|_:String| Msg::GetApi);
        TodoItemComponent {
            api : Default::default(),
            fetch_task: None,
            link,
            props
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
                self.link.send_message(Msg::SetApiFetchState(FetchAction::Fetching));
                ConsoleService::log("fetch");
                let request = RequestHelper::get(self.props.todo_list.id);
                let callback = self.link.callback(|res : Response<Json<Result<ApiResponse, anyhow::Error>>>| {
                    let Json(data) = res.into_body();
                    Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                });
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
                        
                    }
                }
                yewtil::fetch::FetchState::Fetching(_) => {
                    html! {
                        
                    }
                }
                yewtil::fetch::FetchState::Fetched(response) => {
                    response.body[0].items.iter().map(|todo_item: &TodoItem| {
                        html! {
                            <div> 
                                <a><b>{&todo_item.title} </b></a>
                            </div>
                        }
                    }).collect()
                    
                    
                }
                yewtil::fetch::FetchState::Failed(_, _) => {html!{<h1>{"ERROR"}</h1>}}
            }
        }
        
        fn rendered(&mut self, _first_render: bool) {
            
            if _first_render {
                ConsoleService::log("ocu render");
                self.update(Msg::GetApi);
            }
            
        }
        
    }


