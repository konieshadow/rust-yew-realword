use web_sys::Node;
use yew::{function_component, html, Html, virtual_dom::VNode};
use yew_hooks::{UseAsyncOptions, use_async_with_options};

use crate::{services::articles::get, hooks::use_user_context};

mod article_actions;
mod article_meta;
mod comment;
mod delete_button;
mod comment_input;
mod comment_list;

use article_meta::ArticleMeta;
use comment_list::CommentList;

#[derive(yew::Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let article = {
        let slug = props.slug.clone();
        use_async_with_options(
            async move { get(&slug).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    if let Some(article) = &article.data {
        let article = &article.article;
        let can_modify = user_ctx.is_authenticated() && user_ctx.username == article.author.username;
        let created_at = article.created_at.format("%B %e, %Y").to_string();

        html! {
            <div class="article-page">
                <div class="banner">
                    <div class="container">
                        <h1>{ &article.title }</h1>
                        <ArticleMeta
                            slug={article.slug.clone()}
                            author={article.author.clone()}
                            can_modify={can_modify}
                            created_at={created_at} />
                    </div>
                </div>
                <div class="container page">
                    <div class="row article-content">
                        <div class="col-xs-12">
                            { view_body(&article.body) }
                            <ul class="tag-list">
                                {for article.tag_list.iter().map(|tag| {
                                    html! {
                                        <li
                                            class="tag-default tag-pill tag-outline">
                                            { tag }
                                        </li>
                                    }
                                })}
                            </ul>
                        </div>
                    </div>
                    <hr />
                    <div class="article-actions">
                    </div>
                    <div class="row">
                        <CommentList slug={props.slug.clone()} />
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

fn view_body(body: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(body);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(html_text.as_str());
    let node = Node::from(div);
    VNode::VRef(node)
}