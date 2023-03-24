#[cfg(feature = "CgPathUnite")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathUnite")]
/// *This icon requires the feature* `CgPathUnite` *to be enabled*.
#[component]
pub fn PathUnite(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15 5H5V15H9V19H19V9H15V5Z" fill="currentColor" /></svg>
   }
}