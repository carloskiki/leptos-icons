#[cfg(feature = "CgMaximize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMaximize")]
/// *This icon requires the feature* `CgMaximize` *to be enabled*.
#[component]
pub fn Maximize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 3H9V5H5V9H3V3Z" fill="currentColor" /><path d="M3 21H9V19H5V15H3V21Z" fill="currentColor" /><path d="M15 21H21V15H19V19H15V21Z" fill="currentColor" /><path d="M21 3H15V5H19V9H21V3Z" fill="currentColor" /></svg>
   }
}