use yew::prelude::*;

enum Msg {}

pub struct Posts;

#[derive(Properties, PartielEq)]
struct PostsProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Posts {
    type Message = Msg;
    type Properties = PostsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        //match msg {
        //    Msg::Example => {
        //         Do something with state
        //         return true if should render
        //    }
        //}
    }

    fn view(&mut self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                 {"Hello from Posts"}
                 { for ctx.props().children.iter() }
            </div>
        }
    }
}
