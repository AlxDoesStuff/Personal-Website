use yew::prelude::*;
use components::pages::main_page::MainPage;
use components::pages::imprint_page::ImprintPage;
use components::pages::cat_page::CatPage;
use yew_router::prelude::*;

mod components;


//Tab Enum
#[derive(PartialEq,Clone)]
pub enum Tabs {
    About,
    Socials,
    Stuff
}

//Page Routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/imprint")]
    Imprint,
    #[at("/cats")]
    CatPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

//Route Switcher
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home  => html! {<MainPage/> },
        Route::Imprint => html! {<ImprintPage/> },
        Route::CatPage => html! {<CatPage/> },
        Route::NotFound => html! {<div><h1>{"AlxDoesStuff"}</h1><h2>{"404 Not Found"}</h2></div>},
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}



fn main() {
    yew::Renderer::<Main>::new().render();
}