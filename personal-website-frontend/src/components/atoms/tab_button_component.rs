use yew::prelude::*;
use yew::Properties;
use crate::Tabs;

//Tab Button Props
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::from("About"))]
    pub button_text: String, //Button Text
    #[prop_or(Tabs::About)]
    pub button_tab: Tabs, //Tab the Button opens
    pub handle_onclick: Callback<Tabs>, //Click event handler for outside the component
}

#[function_component]
pub fn TabButtonComponent(props: &Props) -> Html {
    let handle_onclick_cloned = props.handle_onclick.clone();
    let button_tab_cloned = props.button_tab.clone();
    let onclick = Callback::from(move |_|{ 
        handle_onclick_cloned.emit(button_tab_cloned.clone()); //Why am I double cloning here? I do not understand
    }); //Button Click Logic
    html! { 
        <div>
            <button type="tab_button" onclick={onclick}>{props.button_text.clone()}</button> 
        </div> 
    } //Button HTML build
} 