use yew::{function_component, html, Callback, use_effect_with_deps};
use yew_hooks::use_async;

use crate::services::comment::delete;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub comment_id: u32,
    pub callback: Callback<u32>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &Props) -> Html {
    let delete_comment = {
        let slug = props.slug.clone();
        let comment_id = props.comment_id;
        use_async(async move { delete(&slug, comment_id).await })
    };

    let onclick = {
        let delete_comment = delete_comment.clone();
        Callback::from(move |_| {
            delete_comment.run();
        })
    };

    {
        use_effect_with_deps(
            move |(callback, comment_id, delete_comment)| {
                if delete_comment.data.is_some() {
                    callback.emit(*comment_id);
                }
                || ()
            },
            (props.callback.clone(), props.comment_id, delete_comment),
        )
    }

    html!{
        <span class="mod-options">
            <i class="ion-trash-a" {onclick}></i>
        </span>
    }
}