use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/work")]
    Work,
    #[at("/games")]
    Games,
    #[at("/posts")]
    Posts,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}
