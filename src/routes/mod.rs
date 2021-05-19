use yew_router::{prelude::Route, Switch};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/lists/{id}"]
    TodoList(i32),
    #[to = "/lists"]
    TodoLists,
    #[to = "/"]
    Home,
}

/// Fix fragment handling problem for yew_router
pub fn fix_fragment_routes(route: &mut Route) {
    let r = route.route.as_str();
    if let Some(index) = r.find('#') {
        route.route = r[index..].to_string();
    } else {
        route.route = "#/".to_string();
    }
}
