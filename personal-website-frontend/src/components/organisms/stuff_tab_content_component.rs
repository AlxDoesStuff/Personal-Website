use yew::prelude::*;
use crate::components::atoms::cat_link_component::CatLinkComponent;

#[function_component]
pub fn StuffTabContentComponent() -> Html {
    html! {
    <div>
        <div>
            <h2>{"Stuff"}</h2>
        </div>
        <div>
            <p>{"Nothing here yet..."}</p>
            <CatLinkComponent/>
        </div>
    </div>
    }
}
