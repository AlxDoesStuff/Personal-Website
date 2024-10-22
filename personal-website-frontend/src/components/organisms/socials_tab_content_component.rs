use stylist::yew::styled_component;
use stylist::style;
use yew::prelude::*;

#[styled_component]
pub fn SocialsTabContentComponent() -> Html {
    //Wrapper properties
    let socials_button_wrapper_style = style!(
        r#"
            height: 9rem;
            padding-right: 3%;
            padding-left: 3%;
            padding-top: 5%;
            gap: 2%;
            justify-content: space-evenly;
            width: 100%;
            display: flex;
            flex-direction: row;
            img {
                height: 100%;
            }
        "#
    ).unwrap();
    //Wrapper wrapper properties
    let socials_button_wrapper_wrapper_style = style!(
        r#"
            width: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: row;
        "#
    ).unwrap();
    //Tab Style
    let tab_style = style!(
        r#"
            
            font-size: 8%;
        "#
    ).unwrap();
    html! {
    <div class = {tab_style}>
        <div class = {socials_button_wrapper_wrapper_style}> 
            <div class = {socials_button_wrapper_style}>
                <a href="https://discord.com/users/805461489116774460">
                    <img src="static/Discord_Logo.svg" alt="Discord"/>
                </a>
                <a href="https://steamcommunity.com/id/alx-1337/">
                    <img src="static/Steam_Logo.svg" alt="Steam"/>
                </a>
                <a href="https://github.com/AlxDoesStuff">
                    <img src="static/Github_Logo.svg" alt="GitHub"/>
                </a>
                <a href="https://open.spotify.com/user/o43ms4aw6tis0shdwdgz48y7h?si=39b9a169c5654c9a">
                    <img src="static/Spotify_Logo.svg" alt="Spotify"/>
                </a>
                <a href="https://www.youtube.com/@alxdoesstuff9352">
                    <img src="static/Youtube_Logo.svg" alt="Youtube" />
                </a>
            </div>
        </div>
    </div>
    }
}
