use stylist::{yew::styled_component, style};
use yew::prelude::*;
mod component;

use component::pages::main_page::MainPage;

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style! {
        r#"
                height: 100%;
                width: 100%;
                overflow: auto;
            "#
    }
    .expect("Style error in list grid");
    html! {
            <div class={stylesheet}>
                <MainPage/>
            </div>
    }
}
