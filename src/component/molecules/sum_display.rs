use stylist::style;
use yew::prelude::*;

#[function_component(SumDisplay)]
pub fn sum_display() -> Html {
    let stylesheet = style!{
        r#"
          background-color: white;
          height: 100px;
          display: grid;
          grid-template-columns: 50% 50%;
        
          div {
            padding: 35px 0;
            font-size: 55px;
          }

          div.align-left {
            text-align:left;
          }

          div.align-right {
            text-align:right;
          }
        "#
    }.expect("Style error in list grid");
    
    html!
    {
        <div class={stylesheet}>
            <div class={"align-left"}>{"Summa"}</div>
            <div class={"align-right"}>{"0"}</div>
        </div>
    }
}
