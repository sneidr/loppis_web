use stylist::style;
use yew::prelude::*;

#[function_component(PreviousSales)]
pub fn previous_sales() -> Html {
    let stylesheet = style!{
        r#"
          background-color: #CCCCCC;
          height: 80px;
          list-style-type: none;
          margin: 0;
          padding: 10px 10px;

          li {
            font-weight: bold;
          }
        "#
    }.expect("Style error in previous sales");
    
    html!
    {
        <ul class={stylesheet}>
            <li>{"Förra gången"}</li>
            <li>{"En till"}</li>
            <li>{"Ännu en"}</li>
        </ul>
    }
}
