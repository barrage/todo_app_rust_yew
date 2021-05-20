use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/lists/{id}"]
    TodoList(i32),
    #[to = "/lists"]
    TodoLists,
    #[to = "/"]
    Home,
}
