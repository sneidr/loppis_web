use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub onkeypress: Callback<KeyboardEvent>,
}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
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
            <input class={stylesheet} id={props.id.clone()} type="text" name={props.name.clone()} placeholder={props.name.clone()} onkeypress={props.onkeypress.clone()}/>
    }
}
