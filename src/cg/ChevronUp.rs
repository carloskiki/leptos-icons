#[cfg(feature = "CgChevronUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgChevronUp")]
/// *This icon requires the feature* `CgChevronUp` *to be enabled*.
#[component]
pub fn ChevronUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M17.6569 16.2427L19.0711 14.8285L12.0001 7.75739L4.92896 14.8285L6.34317 16.2427L12.0001 10.5858L17.6569 16.2427Z" fill="currentColor" /></svg>
   }
}