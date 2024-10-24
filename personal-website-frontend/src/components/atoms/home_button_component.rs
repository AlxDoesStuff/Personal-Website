use stylist::yew::styled_component;
use yew::prelude::*;
use stylist::style;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onclick: Callback<()>, //aaaaaaaaaaa
}

#[styled_component]
pub fn HomeButtonComponent(props: &Props) -> Html {
    //Style
    let button_style = style!(
        r#"         
            background: transparent;
            border: none;
            color: white;
            text-align: center;
            text-decoration: none;
            font-size: 160%;
            font-family: 'Roboto Mono';
            font-weight: bold;
            cursor: pointer;
        "#
    ).unwrap();
    let handle_onclick = props.handle_onclick.clone();
    let onclick = Callback::from(move |_|{ 
        handle_onclick.emit(());
    }); //Button Click Logic
    //Render either the colored or current, unclickable button
    html! { 
        <div>
            <button class = {button_style} onclick={onclick}>{"AlxDoesStuff"}</button> //If this is the button for the current tab
        </div> 
    }
}