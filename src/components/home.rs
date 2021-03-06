use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{components::RouterAnchor};

use crate::routes::AppRoute;

pub struct HomeComponent {
    
}
#[derive(Properties, Clone)]
pub struct Props {}
pub enum Msg {
    Ignore,
}

impl Component for HomeComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HomeComponent {
            
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container" style="height: 400px; width: 100%">
                <div class="row" style="justify-content: center">
                    <div  style="height: 100px"></div>
                </div>
                <div class="row" style="justify-content: center">
                    <h2  style=" color: #17252a"> {"Welcome to my Todo app!"} </h2>
                </div>
                <div class="row" style="justify-content: center">
                    <h4  style="color: #feffff "> {"Made with Rust/Yew"} </h4>
                </div>
                <div class="row" style="justify-content: center">
                    <div  style="height: 100px"></div>
                </div>
                <div class="row" style="justify-content: center">
                    <p >   {"This application was made with a single goal, to research what Yew can and cannot do."}</p>
                </div>
                <div class="row" style="justify-content: center">
                    <p > {"I want to check this out, show me my todo lists!"} </p>
                </div>
                <div class="row" style="justify-content: center">
                    <RouterAnchor<AppRoute> route=AppRoute::TodoLists>
                        <h6 class="btn btn-dark nav-link">{"Gooo!"}</h6>
                    </RouterAnchor<AppRoute>>
                </div>
            </div>
        }
    }
}
