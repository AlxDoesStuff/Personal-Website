use yew::prelude::*;
use yew::Properties;
use super::super::super::Tabs;

//Tab Button Props
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::from("About"))]
    pub button_text: String,
    #[prop_or(Tabs::About)]
    pub button_tab: Tabs,
}

#[function_component]
pub fn TabButtonComponent(&Props {ref button_text, ref button_tab}: &Props) -> Html {
    html! { <div> <>{"This Component has the Text: "}{button_text}</> </div> }   
}   