use routes::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod routes;
use page::Page;
mod page;
use home::*;
mod compressed_posts;
mod dark;
use dark::*;
mod hero;
mod home;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Games => html! { <Page><h1>{"Games"}</h1> </Page>},
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
        </DarkMode>
    }
}

fn main() {
    yew::start_app::<Main>();
}
