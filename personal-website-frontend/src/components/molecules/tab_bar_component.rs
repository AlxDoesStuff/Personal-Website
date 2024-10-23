use stylist::yew::styled_component;
use stylist::style;
use yew::prelude::*;
use yew::Properties;
use crate::Tabs;
use crate::components::atoms::tab_button_component::TabButtonComponent;
use crate::components::atoms::imprint_link_component::ImprintLinkComponent;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_changetab: Callback<Tabs>, //Click event handler for outside the component
    pub current_tab: Option<Tabs>, //Current Tab
}

#[styled_component]
pub fn TabBarComponent(props: &Props) -> Html {
    //CSS
    let tab_wrapper_style = style!(
        r#"
            justify-content: space-evenly;
            align-items: center;
            width: 85%;
            display: flex;
            flex-direction: row;
        "#
    ).unwrap();
    let tab_bar_style = style!(
        r#"
            height: 10%;
            justify-content: space-between;
            width: 100%;
            display: flex;
            flex-direction: row;
            padding-left: 2%;
            padding-top: 2%;
            padding-bottom: 10%;
            h1 {
                display: flex;
                flex-direction: column;
                width: 15%
                font-size: 28px;
                justify-content: center;
                margin: 0;
            }
        "#
    ).unwrap();
    //Cloned handler
    let handle_changetab_cloned = props.handle_changetab.clone();
    let current_tab = props.current_tab.clone();
    //Pass up Tab Button Clicks
    let tab_button_clicked = Callback::from(move |tab_clicked|{
        handle_changetab_cloned.emit(tab_clicked);
    });
    html! {
    <div class = {tab_bar_style}>
        <h1>{"AlxDoesStuff"}</h1>
        <div class = {tab_wrapper_style}>
            <TabButtonComponent button_text = {String::from("About")} button_tab = {Tabs::About} handle_onclick = {tab_button_clicked.clone()} current_tab = {current_tab.clone()}/> //About Tab Button
            <TabButtonComponent button_text = {String::from("Socials")} button_tab = {Tabs::Socials} handle_onclick = {tab_button_clicked.clone()} current_tab = {current_tab.clone()}/> //Socials Tab Button
            <TabButtonComponent button_text = {String::from("Stuff")} button_tab = {Tabs::Stuff} handle_onclick = {tab_button_clicked.clone()} current_tab = {current_tab.clone()}/> //Stuff Tab Button
            <ImprintLinkComponent/>
        </div>
       
    </div>
    }
}
