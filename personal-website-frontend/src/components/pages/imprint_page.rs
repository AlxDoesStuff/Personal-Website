use yew::prelude::*;
use crate::components::molecules::tab_bar_component::TabBarComponent;

use crate::Tabs;


#[function_component]
pub fn ImprintPage() -> Html {
    //Current Tab
    let tab_state = use_state(||Tabs::About);
    let cloned_tab_state = tab_state.clone();
    //Tab changes
    let tab_changed = Callback::from(move |tab|{
        cloned_tab_state.set(tab);
    });
    //Html
    html! {
    <div>
        <TabBarComponent handle_changetab = {tab_changed.clone()}/> //Tab Bar
        <>{"Imprint page!!!1"}</>
    </div>
    }
}

    