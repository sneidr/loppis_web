use stylist::{yew::styled_component, Style};
use yew::prelude::*;
mod component;

use component::pages::main_page::MainPage;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={stylesheet}>
            <MainPage/>
        </div>
    }
}
