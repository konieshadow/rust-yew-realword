use yew::{function_component, html, use_state, use_effect_with_deps, Callback};
use yew_hooks::use_async;

use crate::hooks::use_user_context;
use crate::services::articles::{all, by_author, by_tag, favorited_by, feed};
use super::list_pagination::ListPagination;
use super::article_preview::ArticlePreview;

#[derive(yew::Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub filter: ArticleListFilter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArticleListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    FavoritedBy(String),
    Feed,
}

#[function_component(ArticleList)]
pub fn article_list(props: &Props) -> Html {
    let user_ctx = use_user_context();

    let current_page = use_state(|| 0u32);
    let article_list = {
        let filter = props.filter.clone();
        let current_page = current_page.clone();

        use_async(async move {
            match filter {
                ArticleListFilter::All => all(*current_page).await,
                ArticleListFilter::ByAuthor(author) => by_author(&author, *current_page).await,
                ArticleListFilter::ByTag(tag) => by_tag(&tag, *current_page).await,
                ArticleListFilter::FavoritedBy(author) => favorited_by(&author, *current_page).await,
                ArticleListFilter::Feed => feed().await,
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                current_page.set(0);
                || ()
            },
            props.filter.clone(),
        );
    }

    {
        let article_list = article_list.clone();
        use_effect_with_deps(
            move |_| {
                article_list.run();
                || ()
            },
            (props.filter.clone(), *current_page),
        );
    }

    let callback = {
        let current_page = current_page.clone();
        Callback::from(move |page| {
            current_page.set(page);
        })
    };

    if let Some(article_list) = &article_list.data {
        if !article_list.articles.is_empty() {
            html! {
                <>
                    {for article_list.articles.iter().map(|article| {
                        html! { <ArticlePreview key={article.slug.clone()} article={article.clone()} auth_user={(*user_ctx).clone()} /> }
                    })}
                    <ListPagination
                        total_count={article_list.articles_count}
                        current_page={*current_page}
                        callback={callback} />
                </>
            }
        } else {
            html! {
                <div class="article-preview">{ "No articles are here... yet." }</div>
            }
        }
    } else {
        html! {
            <div class="article-preview">{ "Loading..." }</div>
        }
    }
}