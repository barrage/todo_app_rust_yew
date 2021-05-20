use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Footer {}
#[derive(Properties, Clone)]
pub struct Props {}
pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = Props;
    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <div class="container">
                <div class="row" style="height: 100px"> </div>
               <div class="row" style="justify-content:center">
                    <p>
                        {"Robert Sudec"} <span style="color: #feffff ">{" @ "} </span> {"Barrage 2021"}
                    </p>
               </div>
            </div></div>
        }
    }
}
