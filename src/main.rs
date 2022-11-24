use routes::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod routes;
use page::Page;
mod page;
use games::*;
use home::*;
mod compressed_posts;
mod dark;
use dark::*;
mod games;
mod hero;
mod home;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Games => html! { <GamesPage />},
        Route::Work => html! { <Page><h1>{"Work"}</h1> </Page>},
        Route::About => html! { <Page><h1>{"About"}</h1> </Page>},
        Route::NotFound => html! { <Page><h1>{"NotFound"}</h1> </Page>},
        Route::Posts => html! { <Page><h1>{"Posts"}</h1> </Page>},
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <DarkMode>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
            <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
        </DarkMode>
    }
}

fn main() {
    yew::start_app::<Main>();
}
