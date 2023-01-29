use stylist::{style, yew::styled_component};
use yew::prelude::*;

use crate::component::atoms::text_input::TextInput;

#[styled_component(InputGrid)]
pub fn input_grid() -> Html {
    let stylesheet = style! {
        r#"
            background-color: #222222;
            width: 100%;
            height: 100%;
            display: grid;
            grid-template-rows: 100px 100px auto 100px auto auto;
        "#
    }
    .expect("Style error in input grid");

    let onkeypress_sellerid = Callback::from(|_event: KeyboardEvent| {});
    let onkeypress_price = Callback::from(|_event: KeyboardEvent| {});
    let onkeypress_cashier = Callback::from(|_event: KeyboardEvent| {});

    html! {
        <div class={stylesheet}>
            <div class={css!("height: 100px; margin: 0;")}>
                <TextInput id = {"CashierTextBox"} name={"Kassör"} onkeypress={onkeypress_cashier}/>
            </div>
            <div class={css!("height: 100px;")}></div>
            <div class={css!("position: relative; left: 25%;")}>
                <TextInput id = {"SellerIdTextBox"} name={"SäljarId"} onkeypress={onkeypress_sellerid}/>
                <TextInput id = {"PriceTextBox"} name ={"Pris"} onkeypress={onkeypress_price}/>
            </div>
            <div></div>
            <div></div>
            <div></div>
        </div>
    }
}
