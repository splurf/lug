use leptos::prelude::*;

#[component]
pub fn A(
    href: &'static str,
    #[prop(optional)] style: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href style=style target="_blank" rel="noopener noreferrer">
            {children()}
        </a>
    }
}
