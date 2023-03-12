#[cfg(feature = "CgStudio")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgStudio")]
/// *This icon requires the feature* `CgStudio` *to be enabled*.
#[component]
pub fn Studio(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M17 13H13V17H17V13Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M3 3H21V21H3V3ZM5 5H19V19H5V5Z" fill="currentColor" /></svg>
   }
}