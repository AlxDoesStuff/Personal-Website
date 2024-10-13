use yew::prelude::*;
use yew::Properties;
use crate::Tabs;
use crate::components::atoms::tab_button_component::TabButtonComponent;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_changetab: Callback<Tabs>, //Click event handler for outside the component
}

#[function_component]
pub fn TabBarComponent(props: &Props) -> Html {
    let handle_changetab_cloned = props.handle_changetab.clone();
    //Pass up Tab Button Clicks
    let tab_button_clicked = Callback::from(move |tab|{
        handle_changetab_cloned.emit(tab);
    });
    html! {
    <div>
        <h1>{"AlxDoesStuff"}</h1>
        <TabButtonComponent button_text = {String::from("About")} button_tab = {Tabs::About} handle_onclick = {tab_button_clicked.clone()}/> //About Tab Button
        <TabButtonComponent button_text = {String::from("Socials")} button_tab = {Tabs::Socials} handle_onclick = {tab_button_clicked.clone()}/> //Socials Tab Button
        <TabButtonComponent button_text = {String::from("Stuff")} button_tab = {Tabs::Stuff} handle_onclick = {tab_button_clicked.clone()}/> //Stuff Tab Button
    </div>
    }
}
