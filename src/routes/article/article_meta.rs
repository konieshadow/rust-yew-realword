use yew::{function_component, html};
use yew_router::components::Link;

use crate::{types::Profile, routes::AppRoute};
use super::article_actions::ArticleActions;

#[derive(yew::Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
    pub can_modify: bool,
    pub author: Profile,
    pub created_at: String,
}

#[function_component(ArticleMeta)]
pub fn article_meta(props: &Props) -> Html {
    html! {
        <div class="article-meta">
            <img src={ props.author.image.clone() } alt={ props.author.username.clone() } />
            <div class="info">
                <Link<AppRoute> to={AppRoute::Profile { username: props.author.username.clone() }} classes="author">
                    { &props.author.username }
                </Link<AppRoute>>
                <span class="date">
                    { &props.created_at }
                </span>
            </div>

            <ArticleActions can_modify={props.can_modify} slug={props.slug.clone()} />
        </div>
    }
}