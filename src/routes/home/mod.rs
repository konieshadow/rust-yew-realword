mod banner;
mod tag;
mod main_view;

use yew::{function_component, html, use_state, Callback};
use banner::Banner;
use main_view::MainView;
use tag::Tags;

#[function_component(Home)]
pub fn home() -> Html {
    let tag = use_state(|| None);
    let callback = {
        let tag = tag.clone();
        Callback::from(move |t| {
            tag.set(Some(t));
        })
    };

    html! {
        <div class="home-page">
            <Banner />
            <div class="container page">
                <div class="row">
                    <MainView tag={(*tag).clone()} />
                    <div class="col-md-3 col-xs-12">
                        <div class="sidebar">
                            <p>{ "Popular Tags" }</p>
                            <Tags {callback} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}