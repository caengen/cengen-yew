use super::routes::Route;
use super::DarkModeToggle;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {}

pub struct Page;

#[derive(Properties, PartialEq)]
pub struct PageProps {
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
        // let link = ctx.link();

        html! {
            <div class={"flex flex-col m-auto max-w-screen-md text-gray-900"}>
                <nav class={"py-10"}>
                    <ol class={"flex"}>
                        <li><RouteLink to={Route::Home}>{"@cengen"}</RouteLink></li>
                        <div class={"ml-auto flex space-x-14"}>
                        <li><RouteLink to={Route::About}>{"about"}</RouteLink></li>
                        <li><RouteLink to={Route::Posts}>{"posts"}</RouteLink></li>
                        <li><RouteLink to={Route::Work}>{"work"}</RouteLink></li>
                        <li><RouteLink to={Route::Games}>{"games"}</RouteLink></li>
                        <li><DarkModeToggle /></li>
                        </div>
                    </ol>
                </nav>
                <main class={"py-10"}>
                    { for ctx.props().children.iter() }
                </main>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RouteLinkProps {
    pub children: Children,
    pub to: Route,
}
#[function_component(RouteLink)]
pub fn route_link(props: &RouteLinkProps) -> Html {
    let route = use_route::<Route>().unwrap_or_default();
    let classes = if route == props.to {
        classes!("text-gray-900", "dark:text-gray-100")
    } else {
        classes!(
            "text-gray-400 ",
            "dark:text-gray-300",
            "hover:text-gray-200",
            "dark:hover:text-gray-400"
        )
    };

    html! {
        <Link<Route> classes={classes} to={props.to.clone()}>{for props.children.iter() }</Link<Route>>
    }
}
