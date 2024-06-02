use yew::{function_component, Html};
use yew_template::template_html;

#[function_component(App)]
pub fn app() -> Html {
    let html = template_html!("frontend/templates/home.html");

    html
}