use yew::prelude::*;
use components::pages::main_page::MainPage;

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
    html! {
    <div>
        <MainPage/> //Display Main Page
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}