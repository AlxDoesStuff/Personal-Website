use yew::prelude::*;
use stylist::style;
use yew_router::prelude::*;
use crate::Route;

#[function_component(CatLinkComponent)]
pub fn cat_link_component() -> Html {
    //CSS
    let divstyle = style!(
        r#"
            font-size: 150%;
            padding-left: 10%;
            padding-top: 4%;
            padding-right: 10%;

        "#
    ).unwrap();
        let fontstyle = style!(
            r#"
                color: #999ca1;
                background: rgb(250,0,255);
                background: linear-gradient(90deg, rgba(250,0,255,1) 0%, rgba(255,119,225,1) 100%);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            "#
        ) .unwrap();
    //HTML
    html! {
        <div class = {divstyle}>
            <Link<Route> classes = {fontstyle} to={Route::CatPage}>{ "Daily Cat!" }</Link<Route>>
        </div>
    }
}