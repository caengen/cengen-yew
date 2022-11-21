use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/docs")]
    Home,
    #[at("/docs/work")]
    Work,
    #[at("/docs/games")]
    Games,
    #[at("/docs/posts")]
    Posts,
    #[at("/docs/about")]
    About,
    #[not_found]
    #[at("/docs/404")]
    NotFound,
}
