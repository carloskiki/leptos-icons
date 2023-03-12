#[cfg(feature = "CgBolt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBolt")]
/// *This icon requires the feature* `CgBolt` *to be enabled*.
#[component]
pub fn Bolt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9 21.5L17.5 13L13 10L15 2.5L6.5 11L11 14L9 21.5Z" fill="currentColor" /></svg>
   }
}