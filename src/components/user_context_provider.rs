use yew::{Children, function_component, use_state, use_effect_with_deps, ContextProvider, UseStateHandle, html};
use yew_hooks::{use_async, use_mount};

use crate::{types::AuthUser, services::{auth::current, requests::{get_token, set_token}}, error::Error};

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(AuthUser::default);
    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(auth_user) = &current_user.data {
                    user_ctx.set(auth_user.user.clone());
                }

                if let Some(error) = &current_user.error {
                    match error {
                        Error::Unauthorized | Error::Forbidden => set_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<AuthUser>> context={user_ctx}>
            { for props.children.iter()}
        </ContextProvider<UseStateHandle<AuthUser>>>
    }
}