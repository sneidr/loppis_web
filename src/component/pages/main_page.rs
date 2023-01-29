use stylist::style;
use yew::prelude::*;

use crate::component::molecules::item_list::ItemList;
use crate::component::atoms::text_input::TextInput;
use crate::component::atoms::colored_button::{ColoredButton, ButtonType};

#[function_component(MainPage)]
pub fn main_page() -> Html {
    let stylesheet = style!{
        r#"
            height: 100vh;
            width: 100vw;
            max-width: 100%;
            max-height: 100%;
            display: grid;
            margin: 0;
            border-style: solid;
            border-color: red;
            grid-template-columns: 1fr 500px;
            grid-template-rows: 30px 1fr 1fr 100px;
            padding: 0px;

            div {
                background-color: #222222;
                border-style: solid;
                border-color: red;
            }

            .list-right {
                grid-row: 2 / 4;
                grid-column: 2 / 3;
            }

            .input-left {
                grid-row: 2 / 3;
                grid-column: 1 / 2;
            }

            .header {
                grid-row: 1 / 2;
                grid-column: 1 / 3;
            }

            .footer {
                grid-row: 4 / 5;
                grid-column: 1 / 3;
            }
        "#
    }.expect("Style error in list grid");
    
    let onkeypress_sellerid = Callback::from(|_event: KeyboardEvent| {});
    let onkeypress_price = Callback::from(|_event: KeyboardEvent| {});

    html! {
        <div class={stylesheet}>
            <div class={classes!("header")}>{"Header"}</div>
            <ItemList class={classes!("list-right")}></ItemList>
            <div class={classes!("input-left")}>
                <TextInput id={"SellerIdBox"} name={"Säljare"} onkeypress={onkeypress_sellerid}/>
                <TextInput id={"PriceBox"} name={"Pris"} onkeypress={onkeypress_price}/>
            </div>
            <div>
                <ColoredButton button_type={ButtonType::Card}>{"Vykort"}</ColoredButton>
                <ColoredButton button_type={ButtonType::Bag}>{"Kasse"}</ColoredButton>
                <ColoredButton button_type={ButtonType::Round}>{"Avrunda"}</ColoredButton>
                <ColoredButton button_type={ButtonType::Reset}>{"Rensa"}</ColoredButton>
            </div>
            <div class={classes!("footer")}>{"Footer"}</div>
        </div>
    }
}