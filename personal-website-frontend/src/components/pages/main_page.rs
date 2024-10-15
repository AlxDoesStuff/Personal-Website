use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::molecules::tab_bar_component::TabBarComponent;
use crate::components::organisms::about_tab_content_component::AboutTabContentComponent;
use crate::components::organisms::socials_tab_content_component::SocialsTabContentComponent;
use crate::components::organisms::stuff_tab_content_component::StuffTabContentComponent;
use crate::Tabs;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab: Tabs, //Click event handler for outside the component
}


#[function_component]
pub fn MainPage(props: &Props) -> Html {
    //Navigator
    let navigator = use_navigator().unwrap();
    //Tab given by props
    let props_tab: &Tabs = &props.tab;
    //Current Tab
    let tab_state = use_state(||props_tab.to_owned());
    let tab_state_cloned = tab_state.clone();
    //Tab changes
    let tab_changed = Callback::from(move |tab_clicked|{
        //Cursed Routing    
        match tab_clicked {
            Tabs::About => { navigator.push(&Route::HomeTab { tabstring: "about".to_string() });},
            Tabs::Socials => { navigator.push(&Route::HomeTab { tabstring: "socials".to_string() });},
            Tabs::Stuff => { navigator.push(&Route::HomeTab { tabstring: "stuff".to_string() });},
        }
        tab_state_cloned.set(tab_clicked);
    });
    //Tab Content display 
    let show_current_tab = move || -> Html{
            match *tab_state {
                Tabs::About => { html! {
                    <AboutTabContentComponent />
                }},
                Tabs::Socials => { html! {
                    <SocialsTabContentComponent />
                }},
                Tabs::Stuff => { html! {
                    <StuffTabContentComponent />
                }},
            }
    };
    //Html
    html! {
    <div>
        <TabBarComponent handle_changetab = {tab_changed.clone()}/> //Tab Bar
        <div>{show_current_tab()}</div> //Tab Content
    </div>
    }
}

    