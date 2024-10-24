use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component]
pub fn AboutTabContentComponent() -> Html {
    let text_box_style = style!(
        r#"
            display: flex;
            width: 96%;
            justify-content: center;
            flex-direction: row;
            padding-right: 2%;
            padding-left: 2%;
        "#
    ).unwrap(); 
    let header_style = style!(
        r#"
            display: flex;
            width: 100%;
            justify-content: center;
            flex-direction: row;
        "#
    ).unwrap(); 
    html! {
    <div>
        <div class = {header_style}>
            <h2>{"About"}</h2>
        </div>
        <div class = {text_box_style}>
            <p>{"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."}</p>
        </div>
    </div>
    }
}
