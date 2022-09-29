use web_sys::{FocusEvent, InputEvent, HtmlInputElement};
use yew::{Callback, function_component, html, use_state, TargetCast, use_effect_with_deps};
use yew_hooks::use_async;

use crate::{types::{CommentInfo, AddComment, CommentBody}, hooks::use_user_context, services::comment::create};
use crate::components::list_errors::ListErrors;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub callback: Callback<CommentInfo>,
}

#[function_component(CommentInput)]
pub fn comment_input(props: &Props) -> Html {
    let create_info = use_state(AddComment::default);
    let user_ctx = use_user_context();

    let create_comment = {
        let request = CommentBody {
            comment: (*create_info).clone(),
        };
        let slug = props.slug.clone();
        use_async(async move { create(&slug, request).await })
    };

    let onsubmit = {
        let create_comment = create_comment.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            create_comment.run();
        })
    };

    let oninput = {
        let create_info = create_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*create_info).clone();
            info.body = input.value();
            create_info.set(info);
        })
    };

    {
        let create_info = create_info.clone();
        let callback = props.callback.clone();
        use_effect_with_deps(
            move |create_comment| {
                if let Some(comment_info) = &create_comment.data {
                    create_info.set(AddComment::default());
                    callback.emit(comment_info.comment.clone());
                }
                || ()
            },
            create_comment.clone(),
        );
    }

    html! {
        <>
            <ListErrors error={create_comment.error.clone()} />
            <form class="card comment-form" {onsubmit}>
                <div class="card-block">
                    <textarea class="form-control"
                        placeholder="Write a comment..."
                        row="3"
                        value={create_info.body.clone()}
                        oninput={oninput}>
                    </textarea>
                </div>
                <div class="card-footer">
                    {if user_ctx.is_authenticated() {
                        html! {
                            <img
                                src={ user_ctx.image.clone() }
                                class="comment-author-img"
                                alt={ user_ctx.username.clone() } />
                        }
                    } else {
                        html! {}
                    }}
                    <button
                            class="btn btn-sm btn-primary"
                            type="submit">
                            { "Post Comment" }
                        </button>
                </div>
            </form>
        </>
    }
}