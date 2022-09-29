use web_sys::MouseEvent;
use yew::{function_component, html, Callback};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::tags::get_all;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<String>,
}

#[function_component(Tags)]
pub fn tags(props: &Props) -> Html {
    let tag_list = use_async_with_options(
        async move { get_all().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(tag_list) = &tag_list.data {
        html! {
            <div class="tag-list">
                {for tag_list.tags.iter().map(|tag| {
                    let onclick = {
                        let tag = tag.clone();
                        let callback = props.callback.clone();
                        Callback::from(
                            move |e: MouseEvent| {
                                e.prevent_default();
                                callback.emit(tag.clone());
                            }
                        )
                    };
                    html! {
                        <a
                            href=""
                            class="tag-default tag-pill"
                            {onclick}>
                            { &tag }
                        </a>
                    }
                })}
            </div>
        }
    } else {
        html! {
            <div>{ "Loading Tags..." }</div>
        }
    }
}