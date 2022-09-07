use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class={classes!("ml-3", "bg-gray-900", "p-8", "h-full")}>
                <nav>
                    <ol class={classes!("text-white", "flex", "space-x-6")}>
                        <li class><a href="">{"Christian Engen"}</a></li>
                        <li><a href="">{"Work"}</a></li>
                        <li><a href="">{"Posts"}</a></li>
                        <li><a href="">{"About"}</a></li>
                    </ol>
                </nav>
                // <button
                //     class={classes!("bg-red-400","text-lg", "font-bold", "w-6", "border-2")}
                //     onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                // <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
