use yew::prelude::*;

use crate::component::molecules::item_list::ItemList;
use crate::component::atoms::text_input::TextInput;
use crate::component::atoms::colored_button::{ColoredButton, ButtonType};

#[function_component(MainPage)]
pub fn main_page() -> Html {
    let onkeypress_sellerid = Callback::from(|_event: KeyboardEvent| {});
    let onkeypress_price = Callback::from(|_event: KeyboardEvent| {});

    html! {
        <div class={"container"}>
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