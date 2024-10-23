use stylist::yew::styled_component;
use yew::prelude::*;
use yew::Properties;
use stylist::style;
use crate::Tabs;

//Tab Button Props
#[derive(Properties, PartialEq)]

pub struct Props {
    #[prop_or(String::from("About"))]
    pub button_text: String, //Button Text
    #[prop_or(Tabs::About)]
    pub button_tab: Tabs, //Tab the Button opens
    pub handle_onclick: Callback<Tabs>, //Click event handler for outside the component
    pub current_tab: Option<Tabs>, //Current selected tab
}

#[styled_component]
pub fn TabButtonComponent(props: &Props) -> Html {
    //Style
    let button_current_style = style!(
        r#"         
            background-color: red;
            border: none;
            color: white;
            text-align: center;
            text-decoration: none;
            display: inline-block;
            font-size: 100%;
        "#
    ).unwrap();
    let button_normal_style = style!(
        r#"         
            background-color: transparent;
            border: none;
            color: white;
            text-align: center;
            text-decoration: none;
            display: inline-block;
            font-size: 100%;
        "#
    ).unwrap();
    //Props
    let handle_onclick_cloned = props.handle_onclick.clone();
    let button_tab_cloned = props.button_tab.clone();
    let current_tab_cloned = props.current_tab.clone();
    let onclick = Callback::from(move |_|{ 
        handle_onclick_cloned.emit(button_tab_cloned);
    }); //Button Click Logic
    //Render either the colored or current, unclickable button
    match current_tab_cloned{
        None =>  {html! { 
            <div>
                    <button class = {button_normal_style} onclick={onclick}>{props.button_text.clone()}</button> //If this is the button for the current tab
            </div> 
        }},
        Some(tab) => {html! { 
            <div>
                if tab == button_tab_cloned {
                    <button class = {button_current_style}>{props.button_text.clone()}</button> //If this is not the button for the current tab
                }else{
                    <button class = {button_normal_style} onclick={onclick}>{props.button_text.clone()}</button>
                }
            </div> 
        }},
    }
//Button HTML build
} 