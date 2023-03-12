#[cfg(feature = "CgPathTrim")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathTrim")]
/// *This icon requires the feature* `CgPathTrim` *to be enabled*.
#[component]
pub fn PathTrim(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 5H15V8H8V15H5V5Z" fill="currentColor" /><path d="M19 9H9V19H19V9Z" fill="currentColor" /></svg>
   }
}