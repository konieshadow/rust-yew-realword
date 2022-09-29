use web_sys::MouseEvent;
use yew::{html, function_component, use_state, use_effect_with_deps, Callback};
use yew_hooks::use_async;
use yew_router::{components::Link, prelude::{use_history, History}};

use crate::{types::{Article, AuthUser}, services::articles::{favorite, unfavorite}, routes::AppRoute};

#[derive(yew::Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub article: Article,
    pub auth_user: AuthUser,
}

#[function_component(ArticlePreview)]
pub fn article_preview(props: &Props) -> Html {
    let history = use_history().expect("should be a history");

    let article = use_state(|| props.article.clone());
    let article_favorite = {
        let article = article.clone();
        use_async(async move {
            if article.favorited {
                unfavorite(&article.slug).await
            } else {
                favorite(&article.slug).await
            }
        })
    };

    {
        let article = article.clone();
        let article_favorite = article_favorite.clone();
        use_effect_with_deps(
            move |article_favorite| {
                if let Some(article_info) = &article_favorite.data {
                    article.set(article_info.article.clone());
                }
                || ()
            },
            article_favorite,
        )
    }

    let favorite_button_class = if article.favorited {
        "btn btn-sm btn-primary"
    } else {
        "btn btn-sm btn-outline-primary"
    };

    let is_authenticated = props.auth_user.is_authenticated();

    let onclick = Callback::from(move |ev: MouseEvent| {
        ev.prevent_default();
        if is_authenticated {
            article_favorite.run();
        } else {
            history.push(AppRoute::Login);
        }
    });

    html!{
        <div class="article-preview">
            <div class="article-meta">
                <img src={article.author.image.clone()} />
                <div class="info">
                    <Link<AppRoute>
                        to={AppRoute::Profile { username: article.author.username.clone() }}
                        classes="author">
                        { &article.author.username }
                    </Link<AppRoute>>
                    <span class="date">
                        { &article.created_at.format("%B %e, %Y") }
                    </span>
                </div>
                <div class="pull-xs-right">
                    <button class={favorite_button_class} {onclick}>
                        <i class="ion-heart"></i> { article.favorites_count }
                    </button>
                </div>
            </div>
            <h1>
                <Link<AppRoute>
                    to={AppRoute::Article { slug: article.slug.clone() }}
                    classes="preview-link">
                { &article.title }
                </Link<AppRoute>>
            </h1>
            <p>{ &article.description }</p>
            <span>
                <Link<AppRoute>
                    to={AppRoute::Article { slug: article.slug.clone() }}
                    classes="preview-link">
                    { "Read more..." }
                </Link<AppRoute>>
            </span>
            <ul class="tag-list">
                {for article.tag_list.iter().map(|tag| {
                    html! {
                        <li class="tag-default tag-pill tag-outline" key={ (&tag).to_string() }>
                            { &tag }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}