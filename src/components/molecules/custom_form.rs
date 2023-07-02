use yew::prelude::*;
use crate::components::atoms::off_key::*;



#[function_component(CustomForm)]
pub fn custom_form() -> Html {   
    let firststate = use_state( || Opacity::Normal); 
    let cloned_firststate = firststate.clone(); 
        let opacity_change = Callback::from(move |kind| {
            cloned_firststate.set(kind);
        });                       

    html! {
        <form>
            <KeyImage opacity={*firststate} handle_onchange={opacity_change} />
        </form>
    }
}