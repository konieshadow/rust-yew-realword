use yew::{Callback, function_component, html};
use yew_router::components::Link;
use super::delete_button::DeleteButton;

use crate::{hooks::use_user_context, types::CommentInfo, routes::AppRoute};

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub comment: CommentInfo,
    pub callback: Callback<u32>,
}

#[function_component(Comment)]
pub fn comment(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let comment = &props.comment;
    let show = user_ctx.is_authenticated() && user_ctx.username == comment.author.username;

    html! {
        <div class="card">
            <div class="card-block">
                <p class="card-text">{ &comment.body }</p>
            </div>
            <div class="card-footer">
                <span class="comment-author">
                    <img src={ comment.author.image.clone() } alt={ comment.author.username.clone() } class="comment-author-img" />
                </span>
                { " " }
                <Link<AppRoute> to={AppRoute::Profile { username: comment.author.username.clone() }} classes="comment-author">
                    { &comment.author.username }                
                </Link<AppRoute>>
                <span class="date-posted">
                    { &comment.created_at.format("%B %e, %y") }
                </span>
                { if show {
                    html! {
                        <DeleteButton
                            slug={props.slug.clone()}
                            comment_id={comment.id}
                            callback={props.callback.clone()}
                            />
                    }   
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}