use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Header {}
#[derive(Properties, Clone)]
pub struct Props {}
pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2> {"To-do app test! Header"} </h2>
            </div>
        }
    }
}
