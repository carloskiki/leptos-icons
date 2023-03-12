#[cfg(feature = "CgBorderRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBorderRight")]
/// *This icon requires the feature* `CgBorderRight` *to be enabled*.
#[component]
pub fn BorderRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M8 16V8H15V5L5 5L5 19H15L15 16H8Z" fill="currentColor" fill-opacity="0.3" /><path d="M17 19L17 5L20 5L20 19H17Z" fill="currentColor" /></svg>
   }
}