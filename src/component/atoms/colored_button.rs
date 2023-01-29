use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum ButtonType
{
    Card,
    Bag,
    Round,
    Reset
}

impl ButtonType  
{
 pub fn to_str(&self) -> String
 {
    match self {
        ButtonType::Card => "card".to_owned(),
        ButtonType::Bag => "bag".to_owned(),
        ButtonType::Round => "round".to_owned(),
        ButtonType::Reset => "reset".to_owned(),
    }
 }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub button_type: ButtonType,
}

#[styled_component(ColoredButton)]
pub fn colored_button(props: &Props) -> Html {
    html! {
        <button class={classes!("custom-button".to_owned(), props.button_type.to_str())}>{props.children.clone()}</button>
    }
}
