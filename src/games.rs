use super::Page;
use yew::prelude::*;

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    html! {
      <Page>
        <div>
          <p class={"pb-12"}>{"A compilation of some games I've made while learning game dev concepts and frameworks."}</p>
          <section class={"pb-8 border-b-2 border-b-gray-400"}>
          <div class={"flex column justify-between"}>
            <h1 class={"text-xl pb-3"}>{"Yet another Tetris clone"}</h1>
            <small class={"text-gray-400"}>{"Aug 2022"}</small>
            </div>
            <div class={"flex row justify-between"}>
              <div class={"pt-4 pr-5"}>
                <h2 class={"text-base"}>{"Written in Rust, made with Macroquad"}</h2>
                <p class={"text-base"}>{"
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
                <ol class={"flex space-x-8 py-4 underline underline-offset-4"}>
                  <li>
                    <a href={"https://caengen.github.io/tetris-rs/"}>{"Play"}</a>
                  </li>
                  <li>
                    <a href={"https://github.com/caengen/tetris-rs"}>{"Github"}</a>
                  </li>
                </ol>
              </div>
              <img class={"w-5/12 rounded-xl"} src="https://github.com/caengen/tetris-rs/blob/master/demo/demo.gif?raw=true" alt="showcase tetris"/>
            </div>
          </section>
        </div>
      </Page>
    }
}
