use web_sys::MouseEvent;
use yew::{function_component, html, use_state, use_effect_with_deps, UseStateHandle, Callback, Html};

use crate::{hooks::use_user_context, components::article_list::{ArticleListFilter, ArticleList}};

#[derive(yew::Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub tag: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Tab {
    All,
    Feed,
    Tag,
}

#[function_component(MainView)]
pub fn main_view(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let tab = use_state(|| {
        if user_ctx.is_authenticated() {
            Tab::Feed
        } else {
            Tab::All
        }
    });

    let filter = use_state(|| {
        if user_ctx.is_authenticated() {
            ArticleListFilter::Feed
        } else {
            ArticleListFilter::All
        }
    });

    {
        let tab = tab.clone();
        let filter = filter.clone();
        use_effect_with_deps(
            move |tag| {
                if let Some(tag) = &tag {
                    tab.set(Tab::Tag);
                    filter.set(ArticleListFilter::ByTag(tag.clone()));
                }
                || ()
            },
            props.tag.clone(),
        );
    }

    {
        let filter = filter.clone();
        use_effect_with_deps(
            move |(tab, tag)| {
                match tab {
                    Tab::Feed => filter.set(ArticleListFilter::Feed),
                    Tab::All => filter.set(ArticleListFilter::All),
                    Tab::Tag => {
                        if let Some(tag) = tag {
                            filter.set(ArticleListFilter::ByTag(tag.clone()));
                        }
                    }
                }
                || ()
            },
            ((*tab).clone(), props.tag.clone()),
        )
    }

    html! {
        <div class="col-md-9 col-xs-12">
            <div class="feed-toggle">
                <ul class="nav nav-pills outline-active">
                    {
                        if user_ctx.is_authenticated() {
                            you_feed_tab(tab.clone())
                        } else {
                            html! {}
                        }
                    }
                    { global_feed_tab(tab.clone()) }
                    { tag_filter_tab(tab.clone(), props) }
                </ul>
            </div>

            <ArticleList filter={(*filter).clone()} />
        </div>
    }
}

fn you_feed_tab(tab: UseStateHandle<Tab>) -> Html {
    let (onclick, class) = get_tab_msg_class(tab, Tab::Feed);

    html! {
        <li class="nav-item">
            <a href=""
                {class}
                {onclick}>
                { "Your Feed" }
            </a>
        </li>
    }
}

fn global_feed_tab(tab: UseStateHandle<Tab>) -> Html {
    let (onclick, class) = get_tab_msg_class(tab, Tab::All);

    html! {
        <li class="nav-item">
            <a
                href=""
                {class}
                {onclick}>
                { "Global Feed" }
            </a>
        </li>
    }
}

fn tag_filter_tab(tab: UseStateHandle<Tab>, props: &Props) -> Html {
    if let Some(tag) = &props.tag {
        let (onclick, class) = get_tab_msg_class(tab, Tab::Tag);

        html! {
            <li class="nav-item">
                <a
                    href=""
                    {class}
                    {onclick}>
                    <i class="ion-pound"></i> { &tag }
                </a>
            </li>
        }
    } else {
        html! {}
    }
}

fn get_tab_msg_class(current_tab: UseStateHandle<Tab>, tab: Tab) -> (Callback<MouseEvent>, String) {
    let class = if *current_tab == tab {
        "nav-link active".to_string()
    } else {
        "nav-link".to_string()
    };

    let callback = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *current_tab != tab {
                current_tab.set(tab.clone());
            }
        })
    };

    (callback, class)
}