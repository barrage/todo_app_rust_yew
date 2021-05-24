use yew::{html,  Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{components::RouterAnchor};

use crate::routes::AppRoute;

pub struct Header {
   
}
#[derive(Properties, Clone)]
pub struct Props {}
pub enum Msg {
    Ignore,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Header {
         
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="row">
                    <ul class="nav col">

                        <li class=" nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::Home>
                                <h4 class="btn btn-dark nav-link ">{"Home"}</h4>
                            </RouterAnchor<AppRoute>>
                        </li>
                        <div style="width: 10px"></div>
                        <li class=" nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::TodoLists>
                                <h4 class="btn btn-dark nav-link">{"Lists"}</h4>
                            </RouterAnchor<AppRoute>>
                        </li>
                    </ul>
                    <div class="col" style="text-align:center">
                        <h4 class="nav-link font-weight-bold"> {"Todo app in Yew"} </h4>
                    </div>
                    <ul class="nav col" style="visibility: hidden">

                        <li class=" nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::Home>
                                <h4 class="btn btn-dark nav-link ">{"Home"}</h4>
                            </RouterAnchor<AppRoute>>
                        </li>
                        <div style="width: 10px"></div>
                        <li class=" nav-item">
                            <RouterAnchor<AppRoute> route=AppRoute::TodoLists>
                                <h4 class="btn btn-dark nav-link">{"Lists"}</h4>
                            </RouterAnchor<AppRoute>>
                        </li>
                    </ul>
                </div>
            </div>
        }
    }
}
