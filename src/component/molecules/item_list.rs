use stylist::style;
use yew::prelude::*;

#[function_component(ItemList)]
pub fn item_list() -> Html {
    let stylesheet = style!{
        r#"
          background-color: white;
          height: calc(100% - 300px);
          width: 100%;
          overflow: auto;
          border: 0;
        "#
    }.expect("Style error in item list");
    
    html!
    {
        <select size={20} class={stylesheet}>
        </select>
    }
}
