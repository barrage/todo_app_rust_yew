use yew::{Bridge,Bridged, Component, ComponentLink, Html, Properties, ShouldRender, html};
use yew_router::{components::RouterAnchor, prelude::RouteAgent};

use crate::routes::AppRoute;

pub struct Header {
    router_agent: Box<dyn Bridge<RouteAgent>>,
}
#[derive(Properties, Clone)]
pub struct Props {}
pub enum Msg {
    Ignore,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {true}
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <ul class="nav">
                <li class="nav-item">
                <RouterAnchor<AppRoute> route=AppRoute::Home>
                    <a class="nav-link active">{"Home"}</a>
                </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::TodoLists>
                        <a class="nav-link">{"Lists"}</a>
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>  
        }
    }
}
