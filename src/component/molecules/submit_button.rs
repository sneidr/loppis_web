use stylist::style;
use yew::prelude::*;

#[function_component(SubmitButton)]
pub fn submit_button() -> Html {
    let stylesheet = style!{
        r#"
          background-color: green;
          height: 99px;
          width: 100%;
          font-size: 70px;
          border: 0;
          padding: 0;
        "#
    }.expect("Style error in list grid");
    
    html!
    {
        <button class={stylesheet}>{"Klar"}</button>
    }
}
