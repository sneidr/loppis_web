use crate::component::atoms::colored_button::{ButtonType, ColoredButton};
use crate::component::atoms::text_input::TextInput;
use crate::component::molecules::item_list::ItemList;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

fn set_focus_to(element: &str) {
    let window = web_sys::window().expect("Hej");
    let document = window.document().expect("Hejhej");
    document
        .query_selector(element)
        .expect("1")
        .expect("2")
        .dyn_ref::<HtmlElement>()
        .expect("3")
        .focus()
        .expect("");
}

#[function_component(MainPage)]
pub fn main_page() -> Html {
    let onkeypress_sellerid = Callback::from(|_event: KeyboardEvent| {
        set_focus_to("#PriceBox");
    });
    let onkeypress_price = Callback::from(|_event: KeyboardEvent| {
        set_focus_to("#SellerIdBox");
    });

    html! {
        <div class={"container"}>
            <div class={classes!("header")}>{"Header"}</div>
            <ItemList class={classes!("list-right")}></ItemList>
            <div class={classes!("input-left")}>
                <TextInput id={"SellerIdBox"} name={"Säljare"} onkeypress={onkeypress_sellerid}/>
                <TextInput id={"PriceBox"} name={"Pris"} onkeypress={onkeypress_price}/>
            </div>
            <div class={classes!("buttons-left")}>
                <ColoredButton button_type={ButtonType::Card}>{"Vykort"}</ColoredButton>
                <ColoredButton button_type={ButtonType::Bag}>{"Kasse"}</ColoredButton>
                <ColoredButton button_type={ButtonType::Round}>{"Avrunda"}</ColoredButton>
                <ColoredButton button_type={ButtonType::Reset}>{"Rensa"}</ColoredButton>
            </div>
            <div class={classes!("footer")}>{"Footer"}</div>
        </div>
    }
}
