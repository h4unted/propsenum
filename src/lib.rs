use yew::prelude::*;

mod components;
use crate::components::molecules::custom_form::CustomForm;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <div>
                <CustomForm />
            </div>
        </>
    }
}