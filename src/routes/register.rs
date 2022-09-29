use web_sys::{FocusEvent, InputEvent, HtmlInputElement};
use yew::{use_state, use_effect_with_deps, function_component, Callback, TargetCast, html};
use yew_hooks::use_async;
use yew_router::components::Link;

use crate::{hooks::use_user_context, types::{NewUser, UserBody}, services::auth::register, components::list_errors::ListErrors};

use super::AppRoute;

#[function_component(Register)]
pub fn register() -> Html {
    let user_ctx = use_user_context();
    let new_user = use_state(NewUser::default);
    let user_register = {
        let new_user = new_user.clone();
        use_async(async move {
            let request = UserBody {
                user: (*new_user).clone(),
            };
            register(request).await
        })
    };

    {
        use_effect_with_deps(
            move |user_register| {
                if let Some(auth_user) = &user_register.data {
                    user_ctx.login(auth_user.user.clone());
                }
                || ()
            },
            user_register.clone(),
        )
    }

    let onsubmit = {
        let user_register = user_register.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_register.run();
        })
    };

    let oninput_username = {
        let new_user = new_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_user).clone();
            info.username = input.value();
            new_user.set(info);
        })
    };

    let oninput_email = {
        let new_user = new_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_user).clone();
            info.email = input.value();
            new_user.set(info);
        })
    };

    let oninput_password = {
        let new_user = new_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_user).clone();
            info.password = input.value();
            new_user.set(info);
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign Up" }</h1>
                        <p class="text-xs-center">
                            <Link<AppRoute> to={AppRoute::Login}>
                                { "Have an accountt?" }
                            </Link<AppRoute>>
                        </p>
                        <ListErrors error={user_register.error.clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Username"
                                        value={new_user.username.clone()}
                                        oninput={oninput_username}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value={new_user.email.clone()}
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={new_user.password.clone()}
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign up" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}