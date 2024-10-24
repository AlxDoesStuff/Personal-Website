use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::style;
use crate::components::molecules::tab_bar_component::TabBarComponent;
use crate::Route;
use crate::Tabs;


#[styled_component]
pub fn ImprintPage() -> Html {
    //CSS
    let page_style = style!(
        r#"
            height: 100svh;
            width: 100svw;
            font-size: 28px;
            font-family: 'Roboto Mono';
        "#
    ).unwrap();
    let horizontal_aligner_style = style!(
        r#"
            display: flex;
            width: 100%;
            justify-content: center;
            flex-direction: row;
        "#
    ).unwrap(); 
    //Tab Routing: see main_page.rs
    let navigator = use_navigator().unwrap();
    let tab_state = use_state(||Tabs::About);
    let tab_changed = Callback::from(move |tab_clicked|{
        tab_state.set(tab_clicked);
        //Cursed Routing    
        match tab_clicked {
            Tabs::About => { navigator.push(&Route::HomeTab { tabstring: "about".to_string() });},
            Tabs::Socials => { navigator.push(&Route::HomeTab { tabstring: "socials".to_string() });},
            Tabs::Stuff => { navigator.push(&Route::HomeTab { tabstring: "stuff".to_string() });},
        }
    });
    //Html
    html! {
        <div class = {page_style}>
            <div class = {horizontal_aligner_style.clone()}>
                <TabBarComponent handle_changetab = {tab_changed.clone()} current_tab = {None}/> //Tab Bar
            </div>
            <div class = {horizontal_aligner_style.clone()}>
                <p>{"Imprint"}</p>
            </div>
        </div>
    }
}

    