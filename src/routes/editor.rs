use web_sys::{FocusEvent, InputEvent, HtmlInputElement, KeyboardEvent};
use yew::{function_component, html, use_state, use_effect_with_deps, Callback, TargetCast};
use yew_hooks::use_async;
use yew_router::prelude::{use_history, History};

use crate::{types::{CreateUpdateArticle, ArticleBody}, services::articles::{get, update, create}};
use crate::components::list_errors::ListErrors;

use super::AppRoute;

#[derive(yew::Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: Option<String>,
}

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let history = use_history().unwrap();
    let error = use_state(|| None);
    let update_info = use_state(CreateUpdateArticle::default);
    let tag_input = use_state(String::default);
    let article_get = props
        .slug
        .clone()
        .map(|slug| use_async(async move { get(&slug).await }));
    
    let article_update = {
        let slug = props.slug.clone();
        let update_info = update_info.clone();
        use_async(async move {
            let request = ArticleBody {
                article: (*update_info).clone(),
            };
            if let Some(slug) = slug {
                update(&slug, request).await
            } else {
                create(request).await
            }
        })
    };

    {
        let article_get = article_get.clone();
        use_effect_with_deps(
            move |slug| {
                if slug.is_some() {
                    if let Some(article_get) = article_get {
                        article_get.run();
                    }
                }
                || ()
            },
            props.slug.clone(),
        )
    }

    {
        let update_info = update_info.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |article_get| {
                if let Some(article_get) = article_get {
                    if let Some(article_info) = &article_get.data {
                        update_info.set(CreateUpdateArticle {
                            title: article_info.article.title.clone(),
                            description: article_info.article.description.clone(),
                            body: article_info.article.body.clone(),
                            tag_list: Some(article_info.article.tag_list.clone()),
                        })
                    };
                    error.set(None);
                }
                || ()
            },
            article_get,
        )
    }

    {
        let error = error.clone();
        use_effect_with_deps(
            move |article_update| {
                if let Some(article_info) = &article_update.data {
                    error.set(None);
                    history.push(AppRoute::Article {
                        slug: article_info.article.slug.clone(),
                    });
                }
                if let Some(e) = &article_update.error {
                    error.set(Some(e.clone()));
                }
                || ()
            },
            article_update.clone(),
        )
    }

    let onsubmit = {
        let article_update = article_update.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            article_update.run();
        })
    };

    let oninput_title = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.title = input.value();
            update_info.set(info);
        })
    };

    let oninput_description = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.description = input.value();
            update_info.set(info);
        })
    };

    let oninput_body = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.body = input.value();
            update_info.set(info);
        })
    };

    let oninput_tag = {
        let tag_input = tag_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            tag_input.set(input.value());
        })
    };

    let onkeypress = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });

    let onkeyup = {
        let update_info = update_info.clone();
        let tag_input = tag_input.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                // Add a new tag when press Enter
                let mut info = (*update_info).clone();
                if let Some(tag_list) = &mut info.tag_list {
                    if !tag_list.contains(&*tag_input) {
                        tag_list.push((*tag_input).clone());
                    }
                } else {
                    info.tag_list = Some(vec![(*tag_input).clone()]);
                }
                update_info.set(info);
                tag_input.set(String::default());
            }
        })
    };

    html! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <ListErrors error={(*error).clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Article Title"
                                        value={update_info.title.clone()}
                                        oninput={oninput_title} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="What's this article about?"
                                        value={update_info.description.clone()}
                                        oninput={oninput_description} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control"
                                        rows="8"
                                        placeholder="Write your article (in maridown)"
                                        value={update_info.body.clone()}
                                        oninput={oninput_body}>
                                    </textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="Enter tags"
                                        value={(*tag_input).clone()}
                                        oninput={oninput_tag}
                                        {onkeypress}
                                        {onkeyup} />
                                    <div class="tag-list">
                                        {
                                            if let Some(tag_list) = &update_info.tag_list.clone() {
                                                html! {for tag_list.iter().map(|tag| {
                                                    let onclick_remove = {
                                                        let tag = tag.clone();
                                                        let update_info = update_info.clone();
                                                        Callback::from(move |_| {
                                                            let mut info =(*update_info).clone();
                                                            if let Some(tag_list) = &mut info.tag_list {
                                                                tag_list.retain(|t| t != &tag);
                                                            }
                                                            update_info.set(info);
                                                        })
                                                    };
                                                    html! {
                                                        <span class="tag-default tag-pill">
                                                            <i class="ion-close-round"
                                                                onclick={onclick_remove}>
                                                            </i>
                                                            { &tag }
                                                        </span>
                                                    }
                                                })}
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </fieldset>
                                <button
                                    class="btn btn-lg pull-xs-right btn-primary"
                                    type="submit"
                                    disabled={article_update.loading}>
                                    { "Publish Article" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}