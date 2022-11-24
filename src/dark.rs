use yew::prelude::*;
use yew_router::prelude::*;

pub enum DarkModeMessage {
    Toggle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DarkModeContext {
    pub dark_mode: bool,
    pub toggle: Callback<DarkModeMessage>,
}

pub struct DarkMode {
    dark_enabled: bool,
}

#[derive(Properties, PartialEq)]
pub struct DarkModeProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for DarkMode {
    type Message = DarkModeMessage;
    type Properties = DarkModeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark_enabled: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DarkModeMessage::Toggle => {
                self.dark_enabled = !self.dark_enabled;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = if self.dark_enabled {
            "transition ease-in-out dark:bg-slate-800 min-h-screen"
        } else {
            "transition ease-in-out min-h-screen"
        };
        let toggle = ctx.link().callback(|message: DarkModeMessage| message);
        let context = DarkModeContext {
            dark_mode: self.dark_enabled,
            toggle,
        };
        html! {
          <ContextProvider<DarkModeContext> context={context}>
            <div class={if self.dark_enabled { "dark min-h-screen" } else {"min-h-screen"}}>
                <div class={class}>
                    { for ctx.props().children.iter() }
                </div>
            </div>
          </ContextProvider<DarkModeContext>>
        }
    }
}

#[function_component(DarkModeToggle)]
pub fn dark_mode_toggle() -> Html {
    let ctx = use_context::<DarkModeContext>().expect("no dark mode context available");
    let content = if ctx.dark_mode { "🌝" } else { "🌚" };
    let onclick = move |_| ctx.toggle.emit(DarkModeMessage::Toggle);
    html! {
        <button onclick={onclick} class={"text-2xl leading-5 rounded-full p-1 bg-transparent opacity-80 hover:opacity-100"}>{content}</button>
    }
}
