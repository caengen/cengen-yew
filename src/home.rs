use super::{compressed_posts::CompressedPosts, hero::Hero, Page};
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <Page>
          <Hero />
          <CompressedPosts />
        </Page>
    }
}
