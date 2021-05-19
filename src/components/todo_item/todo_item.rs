use super::super::todo_list::api::TodoList;
use super::api::ApiResponse;
use super::api::RequestHelper;
use super::api::TodoItem;
use super::check_todo_item::CheckTodoItemComponent;
use super::delete_todo_item::DeleteTodoItemComponent;
use super::insert_todo_item::InsertTodoItemComponent;
use crate::components::{footer::Footer, todo_list::api::TodoListWithItems};
use yew::virtual_dom::vnode::VNode;
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
use yewtil::fetch::{Fetch, FetchAction};

pub struct TodoItemComponent {
    api: Fetch<Request<ApiResponse<TodoListWithItems>>, ApiResponse<TodoListWithItems>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    props: Props,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub todo_list: i32,
}

pub enum Msg {
    SetApiFetchState(FetchAction<ApiResponse<TodoListWithItems>>),
    GetApi,
}
impl TodoItemComponent {
    fn reload_data(&mut self) {
        self.update(Msg::GetApi);
    }
}
impl Component for TodoItemComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.callback(|_: String| Msg::GetApi);
        TodoItemComponent {
            api: Default::default(),
            fetch_task: None,
            link,
            props,
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

                let request = RequestHelper::get_items(self.props.todo_list);
                let callback = self.link.callback(
                    |res: Response<Json<Result<ApiResponse<TodoListWithItems>, anyhow::Error>>>| {
                        let Json(data) = res.into_body();
                        Msg::SetApiFetchState(FetchAction::Fetched(data.unwrap()))
                    },
                );

                let task = FetchService::fetch(request, callback).unwrap();
                self.fetch_task = Some(task);

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        self.props = _props;
        true
    }

    fn view(&self) -> yew::Html {
        let callback = self.link.callback(|m: Msg| m);
        html! {
            <div style="margin-left:40px">


                {
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
                            html! {
                                <table>
                                    {response.body[0].items.iter().map(|todo_item: &TodoItem| {

                                        html! {
                                            <tr>
                                                <td><b>{&todo_item.title} </b></td>
                                                <td><CheckTodoItemComponent todo_item=todo_item.clone() refresh=callback.clone()/></td>
                                                <td><DeleteTodoItemComponent todo_item=todo_item.clone() refresh=callback.clone()/></td>
                                            </tr>
                                        }
                                    }).collect::<VNode>()}
                                    <tr>
                                        <td>
                                        <InsertTodoItemComponent todo_list=self.props.todo_list refresh=callback.clone()/>
                                        </td>
                                    </tr>
                                </table>
                            }




                        }
                        yewtil::fetch::FetchState::Failed(_, _) => {html!{<h1>{"ERROR"}</h1>}}
                    }
                }



            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.update(Msg::GetApi);
        }
    }
}
