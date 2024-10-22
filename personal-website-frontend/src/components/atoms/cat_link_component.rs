use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(CatLinkComponent)]
pub fn cat_link_component() -> Html {
    html! {
        <div>
            <Link<Route> to={Route::CatPage}>{ "Daily Cat!" }</Link<Route>>
        </div>
    }
}