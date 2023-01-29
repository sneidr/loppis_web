/*use crate::component::atoms::text_input::TextInput;
use gloo::console::log;
use gloo::{events::EventListenerOptions, utils::document};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let onsellerkeypress = Callback::from(|event: KeyboardEvent| {
        if event.key() == "Enter" {
            let doc = document();
            if let Ok(Some(pb)) = doc.query_selector("input#PriceBox") {
                if let Some(element) = pb.dyn_ref::<HtmlElement>() {
                    element.focus();
                } else {
                    log!("No element!");
                }
            } else {
                log!("No PriceBox");
            }
        } else {
            log!("No document");
        }
    });
    let onpricekeypress = Callback::from(|event: KeyboardEvent| {
        if event.key() == "Enter" {
            let doc = document();
            if let Ok(Some(pb)) = doc.query_selector("input#SellerIdBox") {
                if let Some(element) = pb.dyn_ref::<HtmlElement>() {
                    element.focus();
                } else {
                    log!("No element!");
                }
            } else {
                log!("No PriceBox");
            }
        } else {
            log!("No document");
        }
    });
    html! {
        <form>
            <TextInput id={"SellerIdBox"} name="Säljare" onkeypress={onsellerkeypress}/>
            <TextInput id={"PriceBox"} name="Pris" onkeypress={onpricekeypress}/>
        </form>
    }
}
*/