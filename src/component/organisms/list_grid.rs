use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::style;

use crate::component::molecules::item_list::ItemList;
use crate::component::molecules::previous_sales::PreviousSales;
use crate::component::molecules::sum_display::SumDisplay;
use crate::component::molecules::submit_button::SubmitButton;

#[styled_component(ListGrid)]
pub fn list_grid() -> Html {
    let stylesheet = style!{
        r#"
          width: 100%;
          height: 100%;
          display: grid
          grid-template-rows: auto auto auto auto;
          grid-gap: 0;
        "#
    }.expect("Style error in list grid");

    html!{
        <div class={stylesheet}>
            <ItemList/>
            <PreviousSales/>
            <SumDisplay/>
            <SubmitButton/>
        </div>
    }
}