#[cfg(feature = "CgChevronLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgChevronLeft")]
/// *This icon requires the feature* `CgChevronLeft` *to be enabled*.
#[component]
pub fn ChevronLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16.2426 6.34317L14.8284 4.92896L7.75739 12L14.8285 19.0711L16.2427 17.6569L10.5858 12L16.2426 6.34317Z" fill="currentColor" /></svg>
   }
}