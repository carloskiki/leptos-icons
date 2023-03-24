#[cfg(feature = "CgSpaceBetween")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSpaceBetween")]
/// *This icon requires the feature* `CgSpaceBetween` *to be enabled*.
#[component]
pub fn SpaceBetween(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M19 5L15 5L15 19H19V17H17L17 7L19 7V5Z" fill="currentColor" /><path d="M5 5L9 5L9 19H5L5 17H7L7 7H5L5 5Z" fill="currentColor" /><path d="M13 7V17H11L11 7H13Z" fill="currentColor" /></svg>
   }
}