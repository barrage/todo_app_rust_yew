#![recursion_limit = "512"]
use std::{panic};
pub mod components;
pub mod routes;
extern crate console_error_panic_hook;
use crate::components::{footer::Footer, todo_item::todo_item::TodoItemComponent};
use crate::components::header::Header;
use crate::components::home::HomeComponent;
use crate::components::todo_list::todo_list::TodoListComponent;

use routes::AppRoute;

use wasm_bindgen::prelude::*;
use yew::{prelude::*, services::ConsoleService};
use yew::events::*;
use yew::services::fetch::{FetchTask};

use yew::{html, Html};

use yew::callback::Callback;
use yew_router::{Switch, prelude::RouteAgent, route::Route, service::RouteService};


pub enum Msg {
    RouteChanged(Route<()>),
    //ChangeRoute(AppRoute),
    /*FetchJSON,
    FetchReady(Result<ApiResponse, Error>),

    Try,*/
}
/*#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub code: i32,
    pub message: String,
    pub body: Vec<TodoList>,
}
impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}*/
/*#[derive(Deserialize, Debug)]
pub struct TodoList {
    id: i32,
    title: String,
    items: Vec<TodoItem>,
}*/

/*#[derive(Serialize, Deserialize, Debug)]
pub struct NewTodoList {
    title: String,
}
impl Display for NewTodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}
impl Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}*/

/*#[derive(Deserialize, Debug)]
pub struct TodoItem {
    id: i32,
    title: String,
    done: bool,
    todo_list_id: i32,
}
impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}*/
pub struct Model {
    link: ComponentLink<Self>,
    //data: Option<ApiResponse>,
    fetch: Option<FetchTask>,
    route_service: RouteService<()>,
    route: Route<()>,
    router_agent: Box<dyn Bridge<RouteAgent>>,

}
impl Model {
    /*fn fetch_json(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<ApiResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Try
                }
            },
        );

        let request = Request::get("http://localhost:8081/todo_lists")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }*/
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::RouteChanged));
        let route_service : RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        /*
        let callback= link.callback( Msg::RouteChanged);
        route_service.register_callback(callback);*/
        
        Self {
            link,
            //data: None,
            fetch: None,
            route_service,
            route,
            router_agent
           
            
            
            
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        /*match msg {
            Msg::FetchJSON => {
                /*let task = self.fetch_json();
                self.fetch = Some(task);*/
                true
            }
            Msg::FetchReady(response) => {
                //self.data = response.map(|data| data).ok();
                true
            }
           

            Msg::Try => true,
        }*/
       match msg {
           Msg::RouteChanged(route) => {
             ConsoleService::log(&format!("{}", route.route));   
            self.route_service.set_route(&route.route, ());
            self.route = route
            },
           /*Msg::ChangeRoute(route) => {
            // This might be derived in the future
            let route_string = match route {
                AppRoute::Home => "/".to_string(),
                AppRoute::TodoLists  => "/lists".to_string(),
                AppRoute::TodoList(id) => format!("/lists/{}",id)
                
            };
            self.route_service.set_route(&route_string, ());
            self.route = Route {
                route: route_string,
                state: (),
            };
        }*/
       }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
     

        html! {
            <>
                <Header/>
               
                <div>
                {
                    match AppRoute::switch(self.route.clone()){
                        Some(AppRoute::TodoList(id)) => html! { <TodoItemComponent todo_list=id />},
                        Some(AppRoute::TodoLists) => html! {<TodoListComponent />},
                        Some(AppRoute::Home) => html! {<HomeComponent/>},
                        None => html! {"none"}
                    }
                }
                </div>
                <Footer/>
            </>

        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
