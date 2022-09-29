use yew::{Html, html, function_component};
use yew_router::components::Link;

use crate::{hooks::use_user_context, routes::AppRoute, types::AuthUser};

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = use_user_context();

    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { "conduit" }
                </Link<AppRoute>>
                {
                    if user_ctx.is_authenticated() {
                        logged_in_view((*user_ctx).clone())
                    } else {
                        logged_out_view()
                    }
                }
            </div>
        </nav>
    }
}

fn logged_out_view() -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                    { "Sign in" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
                    { "Sign ip" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}

fn logged_in_view(auth_user: AuthUser) -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::EditorCreate} classes="nav-link">
                    { "New Post" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Settings} classes="nav-link">
                    { "Settings" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Profile { username: auth_user.username.clone() }} classes="nav-link">
                    { &auth_user.username }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}