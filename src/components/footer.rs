use yew::{html, function_component};
use yew_router::components::Link;

use crate::routes::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="logo-font">
                    { "conduit" }
                </Link<AppRoute>>
                <span class="attribution">
                    { "@ 2019. An interactive learning project from "}
                    <a href="https://thinkster.io">{ "Thinkster" }</a>
                    { ". Code licensed under  MIT." }
                </span>
            </div>
        </footer>
    }
}