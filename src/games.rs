use super::Page;
use yew::prelude::*;

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    html! {
      <Page>
        <div>
          <h1 class={"pb-12"}>{"A compilation of some games I've made while learning game dev concepts and frameworks."}</h1>
          <section class={"pb-12 mb-12 border-b-2 border-b-gray-400"}>
            <header class={"flex column justify-between"}>
              <h2 class={"text-xl pb-3"}>{"Yet another Tetris clone"}</h2>
              <small class={"text-gray-400"}>{"Aug 2022"}</small>
            </header>
            <div class={"flex row justify-between"}>
              <div class={"pt-4 pr-5"}>
                <h2 class={"text-base"}>{"Made with Macroquad in Rust"}</h2>
                <p class={"text-base"}>{"
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
                <ol class={"flex space-x-6 py-4 underline underline-offset-4 text-base text-gray-400"}>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://caengen.github.io/tetris-rs/"}>{"Play"}</a>
                  </li>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://github.com/caengen/tetris-rs"}>{"Github"}</a>
                  </li>
                </ol>
              </div>
              <img class={"w-5/12 rounded-xl"} src="https://github.com/caengen/tetris-rs/blob/master/demo/demo.gif?raw=true" alt="showcase tetris"/>
            </div>
          </section>
          <section class={"pb-12 mb-12 border-b-2 border-b-gray-400"}>
            <header class={"flex column justify-between"}>
              <h2 class={"text-xl pb-3"}>{"Asteroids in Rust"}</h2>
              <small class={"text-gray-400"}>{"Jul 2022"}</small>
            </header>
            <div class={"flex row justify-between"}>
              <div class={"pt-4 pr-5"}>
                <h2 class={"text-base"}>{"Made with Macroquad"}</h2>
                <p class={"text-base"}>{"
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
                <ol class={"flex space-x-6 py-4 underline underline-offset-4 text-base text-gray-400"}>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://caengen.github.io/asteroid-rs/"}>{"Play"}</a>
                  </li>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://github.com/caengen/asteroid-rs"}>{"Github"}</a>
                  </li>
                </ol>
              </div>
              <img class={"w-5/12 rounded-xl"} src="https://github.com/caengen/asteroid-rs/blob/master/demo/demo.gif?raw=true" alt="showcase asteroids"/>
            </div>
          </section>
          <section class={"pb-12 mb-12 border-b-2 border-b-gray-400"}>
            <header class={"flex column justify-between"}>
              <h2 class={"text-xl pb-3"}>{"Flappy Bird"}</h2>
              <small class={"text-gray-400"}>{"Jun 2022"}</small>
            </header>
            <div class={"flex row justify-between"}>
              <div class={"pt-4 pr-5"}>
                <h2 class={"text-base"}>{"Everyone's favorite terrible game"}</h2>
                <p class={"text-base"}>{"
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
                <ol class={"flex space-x-6 py-4 underline underline-offset-4 text-base text-gray-400"}>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://caengen.github.io/flappy-bird-rs/"}>{"Play"}</a>
                  </li>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://github.com/caengen/flappy-bird-rs"}>{"Github"}</a>
                  </li>
                </ol>
              </div>
              <img class={"w-5/12 rounded-xl"} src="https://github.com/caengen/flappy-bird-rs/blob/master/demo/flappy.gif?raw=true" alt="showcase asteroids"/>
            </div>
          </section>
          <section class={"pb-12 mb-12 border-b-2 border-b-gray-400"}>
            <header class={"flex column justify-between"}>
              <h2 class={"text-xl pb-3"}>{"Pong in Rust!"}</h2>
              <small class={"text-gray-400"}>{"May 2022"}</small>
            </header>
            <div class={"flex row justify-between"}>
              <div class={"pt-4 pr-5"}>
                <h2 class={"text-base"}>{"First game of the year"}</h2>
                <p class={"text-base"}>{"
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
                <ol class={"flex space-x-6 py-4 underline underline-offset-4 text-base text-gray-400"}>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://caengen.github.io/pong-rs/"}>{"Play"}</a>
                  </li>
                  <li>
                    <a class={"dark:hover:text-gray-100 hover:text-gray-600"} href={"https://github.com/caengen/pong-rs"}>{"Github"}</a>
                  </li>
                </ol>
              </div>
              <img class={"w-7/12 rounded-xl"} src="https://github.com/caengen/pong-rs/blob/master/demo/demo.gif?raw=true" alt="showcase asteroids"/>
            </div>
          </section>
        </div>
      </Page>
    }
}
