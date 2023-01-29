use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props
{
    pub class: Classes,
}


#[function_component(ItemList)]
pub fn item_list(props: &Props) -> Html {
    // TODO: Style
    // let stylesheet = style!{
    //     r#"
    //         select {
    //             background-color: white;
    //             height: calc(100% - 300px);
    //             width: 100%;
    //             overflow: auto;
    //             border: 0;
    //         }
    //     "#
    // }.expect("Style error in item list");
    
    html!
    {
        <select size={20} class={props.class.clone()}>
        </select>
    }
}
