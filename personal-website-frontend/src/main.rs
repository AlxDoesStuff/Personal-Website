use yew::prelude::*;
use components::atoms::tab_button_component::TabButtonComponent;

mod components;


//Tab Enum
#[derive(PartialEq,Clone)]
pub enum Tabs {
    About,
    Socials,
    Stuff
}


#[function_component]
fn App() -> Html {
    let tab_state = use_state(||"no tab".to_owned());
    let cloned_tab_state = tab_state.clone();
    //On click, set "tab_state", which is the current tab as a string slice based on which button is pressed
    let tab_button_clicked = Callback::from(move |tab|{
        match tab {
            Tabs::About => cloned_tab_state.set(String::from("About")),
            Tabs::Socials => cloned_tab_state.set(String::from("Socials")),
            Tabs::Stuff => cloned_tab_state.set(String::from("Stuff")),
        }
    });
    
    html! {
    <>
        <TabButtonComponent button_text = {String::from("About")} button_tab = {Tabs::About} handle_onclick = {tab_button_clicked.clone()}/> //About Tab Button
        <TabButtonComponent button_text = {String::from("Socials")} button_tab = {Tabs::Socials} handle_onclick = {tab_button_clicked.clone()}/> //Socials Tab Button
        <TabButtonComponent button_text = {String::from("Stuff")} button_tab = {Tabs::Stuff} handle_onclick = {tab_button_clicked.clone()}/> //Stuff Tab Button

        <>{&*tab_state}</> //Display Current Tab
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}