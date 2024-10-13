use yew::prelude::*;
use crate::components::molecules::tab_bar_component::TabBarComponent;
use crate::components::organisms::about_tab_content_component::AboutTabContentComponent;
use crate::components::organisms::socials_tab_content_component::SocialsTabContentComponent;
use crate::components::organisms::stuff_tab_content_component::StuffTabContentComponent;
use crate::Tabs;

#[function_component]
pub fn MainPage() -> Html {
    //Current Tab
    let tab_state = use_state(||Tabs::About);
    let cloned_tab_state = tab_state.clone();
    //Tab changes
    let tab_changed = Callback::from(move |tab|{
        cloned_tab_state.set(tab);
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

    