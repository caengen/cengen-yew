use routes::*;
use yew::prelude::*;
use yew_router::prelude::*;
mod routes;

enum Msg {}

struct Page;

#[derive(Properties, PartialEq)]
struct PageProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Page {
    type Message = Msg;
    type Properties = PageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {
        //     Msg::AddOne => {
        //         self.value += 1;
        //         // the value has changed so we need to
        //         // re-render for it to appear on the page
        //         true
        //     }
        // }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class={classes!("ml-3", "bg-red-100", "p-", "h-")}>
                <nav>
                    <ol class={classes!("text-white", "flex", "space-x-6")}>
                        <li ><Link<Route> to={Route::Home}>{"Christian Engen"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Work}>{"Work"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Posts}>{"Posts"}</Link<Route>></li>
                        <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                    </ol>
                </nav>
                <main>
                    { for ctx.props().children.iter() }
                </main>
                // <button
                //     class={classes!("bg-red-400","text-lg", "font-bold", "w-6", "border-2")}
                //     onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                // <p>{ self.value }</p>
            </div>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Page><h1>{"Home"}</h1></Page> },
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
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
