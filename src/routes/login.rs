use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, use_effect_with_deps, Callback, FocusEvent, InputEvent, TargetCast};
use yew_hooks::use_async;
use yew_router::components::Link;

use crate::{hooks::use_user_context, types::{UserBody, LoginUser}, services::auth::login};
use crate::components::list_errors::ListErrors;

use super::AppRoute;

#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let login_user = use_state(LoginUser::default);
    let user_login = {
        let login_user = login_user.clone();
        use_async(async move {
            let request = UserBody {
                user: (*login_user).clone(),
            };
            login(request).await
        })
    };

    use_effect_with_deps(
        move |login_user| {
            if let Some(auth_user) = &login_user.data {
                user_ctx.login(auth_user.user.clone());
            }
            || ()
        },
        user_login.clone(),
    );

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_login.run();
        })
    };

    let oninput_email = {
        let login_user = login_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_user).clone();
            info.email = input.value();
            login_user.set(info);
        })
    };

    let oninput_password = {
        let login_user = login_user.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_user).clone();
            info.password = input.value();
            login_user.set(info);
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-sx-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        <p class="text-xs-center">
                            <Link<AppRoute> to={AppRoute::Register}>
                                { "Need an account?" }
                            </Link<AppRoute>>
                            <ListErrors error={user_login.error.clone()} />
                            <form {onsubmit}>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value={login_user.email.clone()}
                                            oninput={oninput_email}
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value={login_user.password.clone()}
                                            oninput={oninput_password}
                                            />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg btn-primary pull-xs-right"
                                        type="submit"
                                        disabled=false>
                                        { "Sign in" }
                                    </button>
                                </fieldset>
                            </form>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}