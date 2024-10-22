use yew::prelude::*;
use stylist::style;
use yew_router::prelude::*;
use crate::Route;

#[function_component(ImprintLinkComponent)]
pub fn imprint_link_component() -> Html {
    let divstyle = style!(
        r#"
            font-size: 75%;
            padding-left: 10%;
            padding-right: 10%;
        "#
    ).unwrap();
    let fontstyle = style!(
        r#"
            color: #999ca1;
        "#
    ).unwrap();
    html! {
        <div class = {divstyle}>
            <Link<Route> classes = {fontstyle} to={Route::Imprint}>{ "Imprint" }</Link<Route>>
        </div>
    }
}