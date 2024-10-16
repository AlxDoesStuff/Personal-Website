use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::molecules::tab_bar_component::TabBarComponent;
use crate::Route;
use crate::Tabs;


#[function_component]
pub fn CatPage() -> Html {
    //Tab Routing: see main_page.rs
    let navigator = use_navigator().unwrap();
    let tab_state = use_state(||Tabs::About);
    let tab_state_cloned = tab_state.clone();
    let tab_changed = Callback::from(move |tab_clicked|{
        tab_state_cloned.set(tab_clicked);
        //Cursed Routing    
        match *tab_state_cloned {
            Tabs::About => { navigator.push(&Route::HomeTab { tabstring: "about".to_string() });},
            Tabs::Socials => { navigator.push(&Route::HomeTab { tabstring: "socials".to_string() });},
            Tabs::Stuff => { navigator.push(&Route::HomeTab { tabstring: "stuff".to_string() });},
        }
    });
    //Html
    html! {
    <div>
        <TabBarComponent handle_changetab = {tab_changed.clone()}/> //Tab Bar
        <>{"Cat page!!!1"}</>
    </div>
    }
}

    