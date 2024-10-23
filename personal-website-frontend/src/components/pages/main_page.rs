use stylist::yew::styled_component;
use stylist::style;
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


#[styled_component]
pub fn MainPage(props: &Props) -> Html {
    //Wrapper properties
    let page_style = style!(
        r#"
            height: 100svh;
            width: 100svw;
            font-size: 28px;
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
    //Navigator
    let navigator = use_navigator().unwrap();
    //Tab given by props
    let props_tab: &Tabs = &props.tab;
    //Current Tab
    //WHAT THE FUCK
    let tab_state = use_state(||props_tab.to_owned());
    let tab_state_button = use_state(||props_tab.to_owned());
    let tab_state_cloned: UseStateHandle<Tabs> = tab_state.clone();
    let tab_state_button_cloned: UseStateHandle<Tabs> = tab_state_button.clone();
    //Tab changes
    let tab_changed = Callback::from(move |tab_clicked|{
        //Cursed Routing    
        match tab_clicked {
            Tabs::About => { navigator.push(&Route::HomeTab { tabstring: "about".to_string() });},
            Tabs::Socials => { navigator.push(&Route::HomeTab { tabstring: "socials".to_string() });},
            Tabs::Stuff => { navigator.push(&Route::HomeTab { tabstring: "stuff".to_string() });},
        }
        //???
        tab_state_cloned.set(tab_clicked);
        tab_state_button_cloned.set(tab_clicked);
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
        <div class = {page_style}>
            <div class = {horizontal_aligner_style.clone()}>
            <TabBarComponent handle_changetab = {tab_changed.clone()} current_tab = {*tab_state_button}/> //Tab Bar
            </div>
            {show_current_tab()} //Tab Content
        </div>
    }
}

    