use super::Page;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Img {
    pub src: String,
    pub alt: String,
    pub class: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct GameItemProps {
    pub title: String,
    pub date: String,
    pub children: Children,
    pub play_href: String,
    pub github_href: String,
    pub img: Img,
}

#[function_component(GameItem)]
pub fn game_item(props: &GameItemProps) -> Html {
    html! {
      <section class={"pb-12 mb-12 border-b-2 border-b-gray-400"}>
        <header class={"flex column justify-between"}>
          <h2 class={"text-xl pb-3"}>{props.title.clone()}</h2>
          <small class={"text-gray-400"}>{props.date.clone()}</small>
        </header>
        <div class={"flex row justify-between"}>
          <div class={"pr-5"}>
            {for props.children.iter() }
            <ol class={"flex space-x-6 py-4 underline underline-offset-4 text-base text-gray-400"}>
              <li>
                <a target="_" class={"dark:hover:text-gray-100 hover:text-gray-600"} href={props.play_href.clone()}>{"Play"}</a>
              </li>
              <li>
                <a target="_" class={"dark:hover:text-gray-100 hover:text-gray-600"} href={props.github_href.clone()}>{"Github"}</a>
              </li>
            </ol>
          </div>
          <img class={format!("w-5/12 rounded-xl {0}", props.img.class.clone())} src={props.img.src.clone()} alt={props.img.alt.clone()}/>
        </div>
      </section>
    }
}

#[function_component(GamesPage)]
pub fn games_page() -> Html {
    html! {
      <Page>
        <div>
          <h1 class={"pb-12"}>{"A compilation of some games I've made while learning game dev concepts and frameworks."}</h1>
          <GameItem
            title={" Asteroids with physics"}
            date={"Current project"}
            play_href={"https://caengen.github.io/asteroids-bevy/"}
            github_href={"https://github.com/caengen/asteroids-bevy"}
            img={Img {
              src: String::from("https://github.com/caengen/asteroids-bevy/blob/master/demo/demo.gif?raw=true"),
              alt: String::from("showcase asteroids"),
              class: String::from("w-5/12"),
            }}
          >
            <h2 class={"text-base"}>{"Made in Bevy"}</h2>
            <p class={"text-base"}>{"Experimenting with writing a custom physics simulation. Work in progress..."}</p>
          </GameItem>
          <GameItem
            title={"Yet another Tetris clone"}
            date={"Aug 2022"}
            play_href={"https://caengen.github.io/tetris-rs/"}
            github_href={"https://github.com/caengen/tetris-rs"}
            img={Img {
              src: String::from("https://github.com/caengen/tetris-rs/blob/master/demo/demo.gif?raw=true"),
              alt: String::from("showcase tetris"),
              class: String::from("w-5/12"),
            }}
          >
            <h2 class={"text-base"}>{"Made with Macroquad in Rust"}</h2>
            <p class={"text-base"}>{"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
          </GameItem>
          <GameItem
            title={"Asteroids in Rust"}
            date={"Jul 2022"}
            play_href={"https://caengen.github.io/asteroid-rs/"}
            github_href={"https://github.com/caengen/asteroid-rs"}
            img={Img {
              src: String::from("https://github.com/caengen/asteroid-rs/blob/master/demo/demo.gif?raw=true"),
              alt: String::from("showcase asteroid"),
              class: String::from("w-5/12"),
            }}
          >
            <h2 class={"text-base"}>{"Made with Macroquad"}</h2>
            <p class={"text-base"}>{"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
          </GameItem>
          <GameItem
            title={"Flappy bird"}
            date={"Jun 2022"}
            play_href={"https://caengen.github.io/flappy-bird-rs/"}
            github_href={"https://github.com/caengen/flappy-bird-rs"}
            img={Img {
              src: String::from("https://github.com/caengen/flappy-bird-rs/blob/master/demo/flappy.gif?raw=true"),
              alt: String::from("showcase flappy-bird"),
              class: String::from("w-5/12"),
            }}
          >
            <h2 class={"text-base"}>{"Everyone's favorite terrible game"}</h2>
            <p class={"text-base"}>{"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
          </GameItem>
          <GameItem
            title={"Pong"}
            date={"May 2022"}
            play_href={"https://caengen.github.io/pong-rs/"}
            github_href={"https://github.com/caengen/pong-rs"}
            img={Img {
              src: String::from("https://github.com/caengen/pong-rs/blob/master/demo/demo.gif?raw=true"),
              alt: String::from("showcase pong"),
              class: String::from("w-5/12"),
            }}
          >
            <h2 class={"text-base"}>{"First game of the year"}</h2>
            <p class={"text-base"}>{"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed massa orci, condimentum quis orci et, tempor tincidunt diam. Nullam dictum, lacus et pulvinar euismod, tortor neque commodo elit."}</p>
          </GameItem>
        </div>
      </Page>
    }
}
