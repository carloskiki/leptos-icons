#[cfg(feature = "CgPathExclude")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathExclude")]
/// *This icon requires the feature* `CgPathExclude` *to be enabled*.
#[component]
pub fn PathExclude(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M5 5H15V9H9V15H5V5ZM9 15V19H19V9H15V15H9Z" fill="currentColor" /></svg>
   }
}