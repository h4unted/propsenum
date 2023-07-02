use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq,)]
pub struct Props {
    pub opacity: Opacity,
    pub handle_onchange: Callback<Opacity>,
    
}

#[derive(PartialEq, Copy, Clone,)]
pub enum Opacity {
    Normal,
    Off,    
}

impl Opacity {
    pub fn to_string(&self) -> String {
        match self {
            Opacity::Normal => "normal".to_owned(),
            Opacity::Off => "off".to_owned(),
        }
    }
}

#[styled_component(KeyImage)]
pub fn key_image(props: &Props) -> Html {    
    let stylesheet = style!(
        r#"
        .normal {
            opacity: 1;
        }

        .off {
            opacity: 0;
        }
    "#
    ) .unwrap();
    let handle_onchange = props.handle_onchange.clone();
    let onclick = Callback::from(move |_| {
        handle_onchange.emit(Opacity::Off);
    }); 

    html! {
        <div class={stylesheet}>
            <img class={props.opacity.to_string()} src="pictures/offkey.png" {onclick} alt="offkey image" />
        </div>
    }
}
