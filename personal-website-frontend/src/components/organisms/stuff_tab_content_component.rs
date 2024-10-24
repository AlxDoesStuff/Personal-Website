use yew::prelude::*;
use stylist::style;
use crate::components::atoms::cat_link_component::CatLinkComponent;

#[function_component]
pub fn StuffTabContentComponent() -> Html {
    let horizontal_aligner_style = style!(
        r#"
            display: flex;
            width: 100%;
            justify-content: center;
            flex-direction: row;
        "#
    ).unwrap(); 
    html! {
    <div>
        <div class = {horizontal_aligner_style.clone()}>
            <p>{"Nothing here yet, except..."}</p>
        </div>
        <div class = {horizontal_aligner_style.clone()}>
            <CatLinkComponent/>
        </div>
    </div>
    }
}
