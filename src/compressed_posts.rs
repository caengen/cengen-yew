use yew::prelude::*;

pub enum Msg {}

pub struct CompressedPosts;

#[derive(Properties, PartialEq)]
pub struct CompressedPostsProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for CompressedPosts {
    type Message = Msg;
    type Properties = CompressedPostsProps;

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
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={"py-10"}>
                 {"Hello from CompressedPosts"}
                 { for ctx.props().children.iter() }
            </div>
        }
    }
}
