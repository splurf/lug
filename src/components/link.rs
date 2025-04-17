use leptos::prelude::*;

#[component]
pub fn A(
    href: &'static str,
    #[prop(optional)] style: &'static str,
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href style=style class=class target="_blank" rel="noopener noreferrer">
            {children()}
        </a>
    }
}
