#[cfg(feature = "CgChevronRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgChevronRight")]
/// *This icon requires the feature* `CgChevronRight` *to be enabled*.
#[component]
pub fn ChevronRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M10.5858 6.34317L12 4.92896L19.0711 12L12 19.0711L10.5858 17.6569L16.2427 12L10.5858 6.34317Z" fill="currentColor" /></svg>
   }
}