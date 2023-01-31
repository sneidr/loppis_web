use stylist::style;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub onkeypress: Callback<KeyboardEvent>,
}

fn validate_input(input: &str) -> bool {
    if let Ok(_num) = input.parse::<i32>() {
        return true;
    }
    false
}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let text_handle = use_state(|| "".to_owned());
    
    let onkeypress = {
        let onkeypress_clone = props.onkeypress.clone();
        let text_handle = text_handle.clone();
        Callback::from(move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            //gloo::console::log!("validate1", value.as_str());
            text_handle.set(value.clone());
            if validate_input(value.as_str()) {
                onkeypress_clone.emit(event);
                return;
            }

            gloo::console::log!("Input is not valid!");
        }
    })};

    let stylesheet = style!(
        r#"
            height:150px;
            width:300px;
            font-size:70px;
            display: inline;
            float: none;
            margin: 20px;
        "#
    )
    .expect("Hej");
    html! {
            <input class={stylesheet} id={props.id.clone()} type="text" name={props.name.clone()} placeholder={props.name.clone()} {onkeypress} value={(*text_handle).clone()}/>
    }
}
