mod article;
mod home;
mod login;
mod register;
mod editor;
mod settings;
mod profile;

use article::Article;
use home::Home;
use login::Login;
use register::Register;
use editor::Editor;
use settings::Settings;
use profile::Profile;
use yew::{html, Html};
use yew_router::Routable;

use self::profile::ProfileTab;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/register")]
    Register,

    #[at("/editor")]
    EditorCreate,

    #[at("/editor/:slug")]
    EditorUpdate { slug: String },

    #[at("/article/:slug")]
    Article { slug: String },

    #[at("/settings")]
    Settings,

    #[at("/:username")]
    Profile { username: String },

    #[at("/:username/favorites")]
    ProfileFavorites { username: String },

    #[not_found]
    #[at("/404")]
    NotFOund,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home /> },
        AppRoute::Login => html! {<Login /> },
        AppRoute::Register => html! {<Register /> },
        AppRoute::Article { slug } => html! { <Article slug={slug.clone()} /> },
        AppRoute::EditorCreate => html! { <Editor /> },
        AppRoute::EditorUpdate { slug } => html!{ <Editor slug={Some(slug.clone())} /> },
        AppRoute::Settings => html! { <Settings /> },
        AppRoute::ProfileFavorites { username } => html! {
            <Profile username={username.clone()} tab={ProfileTab::FavoritedBy} />
        },
        AppRoute::Profile { username } => html! {
            <Profile username={username.clone()} tab={ProfileTab::ByAuthor} />
        },
        _ => html! { "Page not found" },
    }
}
