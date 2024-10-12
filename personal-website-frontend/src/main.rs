use yew::prelude::*;
use components::atoms::tab_button_component::TabButtonComponent;

mod components;


//Tab Enum
#[derive(PartialEq)]
pub enum Tabs {
    About,
    Socials,
    Stuff
}


#[function_component]
fn App() -> Html {
    html! { 
    <>
        <TabButtonComponent button_text = {String::from("About")} button_tab = {Tabs::About} />
    </>
    }
}

fn main() {
    let currentTab: Tabs;
    yew::Renderer::<App>::new().render();
}