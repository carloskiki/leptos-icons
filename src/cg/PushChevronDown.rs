#[cfg(feature = "CgPushChevronDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPushChevronDown")]
/// *This icon requires the feature* `CgPushChevronDown` *to be enabled*.
#[component]
pub fn PushChevronDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 7.41421L6.41421 6L12.0711 11.6569L17.7279 6L19.1421 7.41421L12.0711 14.4853L5 7.41421Z" fill="currentColor" /><path d="M19 16.3432H5V18.3432H19V16.3432Z" fill="currentColor" /></svg>
   }
}