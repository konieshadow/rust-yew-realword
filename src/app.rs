use yew::{function_component, html};
use yew_router::BrowserRouter;
use yew_router::Switch;
use crate::components::{
    footer::Footer, header::Header, user_context_provider::UserContextProvider,
};
use crate::routes::AppRoute;
use crate::routes::switch;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <Header />
                <Switch<AppRoute> render={Switch::render(switch)} />
                <Footer />
            </BrowserRouter>
        </UserContextProvider>
    }
}
