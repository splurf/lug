use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="Linux Users Group" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { {NotFound} }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
