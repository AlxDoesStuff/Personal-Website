use yew::prelude::*;
use stylist::yew::styled_component;
use components::pages::main_page::MainPage;
use components::pages::imprint_page::ImprintPage;
use components::pages::cat_page::CatPage;
use yew_router::prelude::*;

mod components;


//Tab Enum
#[derive(PartialEq,Clone,Copy)]
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
    #[at("/*tabstring")]
    HomeTab { tabstring: String },
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
        Route::Home => html! {<MainPage tab = {Tabs::About}/>}, 
        Route::HomeTab {tabstring}  =>  html! {
            match &*tabstring.to_lowercase() {
                "about" => html! {<MainPage tab = {Tabs::About}/>},
                "socials" => html! {<MainPage tab = {Tabs::Socials}/>},
                "stuff" => html! {<MainPage tab = {Tabs::Stuff}/>},
                _ => html! {<MainPage tab = {Tabs::About}/>},
            }
        },
        Route::Imprint => html! {<ImprintPage/> },
        Route::CatPage => html! {<CatPage/> },
        Route::NotFound => html! {<div><h1>{"AlxDoesStuff"}</h1><h2>{"404 Not Found"}</h2></div>},
    }
}

#[styled_component(Main)]
fn app() -> Html {
    //Html
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}



fn main() {
    yew::Renderer::<Main>::new().render();
}