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
use yew::prelude::*;


use yew::{html, Html};


use yew_router::{Switch, prelude::RouteAgent, route::Route, service::RouteService};


pub enum Msg {
    RouteChanged(Route<()>),
}

pub struct Model {
    #[allow(unused)]
    link: ComponentLink<Self>,
    #[allow(unused)]
    route_service: RouteService<()>,
    route: Route<()>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,

}
impl Model {
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::RouteChanged));
        let route_service : RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        
        Self {
            link,
            route_service,
            route,
            router_agent
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        
       match msg {
           Msg::RouteChanged(route) => {  
            self.route_service.set_route(&route.route, ());
            self.route = route
            },
       }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header/>
                <div>
                {
                    match AppRoute::switch(self.route.clone()){
                        Some(AppRoute::TodoList(id)) => html! { <TodoItemComponent todo_list=id/>},
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
