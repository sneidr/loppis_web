use stylist::style;
use yew::prelude::*;

use crate::component::organisms::input_grid::InputGrid;
use crate::component::organisms::list_grid::ListGrid;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    let stylesheet = style!{
        r#"
            height: 100%;
            width: 100%;
            display: grid;
            grid-template-columns: auto 500px;
            padding: 0px;
            overflow: auto;
        "#
    }.expect("Style error in list grid");
    
    html! {
        <div class={stylesheet}>
            <InputGrid/>
            <ListGrid/>
        </div>
    }
}