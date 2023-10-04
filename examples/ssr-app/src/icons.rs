use leptos::*;
use leptos_icons::AiIcon::*;
use leptos_icons::*;

#[component]
pub fn Icons() -> impl IntoView {
    view! {
        <Icon icon=Icon::from(AiAppleFilled) />
    }
}
