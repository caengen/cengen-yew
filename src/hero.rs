use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div class={"py-12 ml-auto"}>
            <h1 class={"text-3xl leading-10"}>{"👋 Hi, my name is Christian. I'm a front end developer.\nCurrently I am passionate about ... game development."}</h1>
        </div>
    }
}
