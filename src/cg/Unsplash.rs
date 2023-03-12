#[cfg(feature = "CgUnsplash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgUnsplash")]
/// *This icon requires the feature* `CgUnsplash` *to be enabled*.
#[component]
pub fn Unsplash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15 4.5H9V8.5H15V4.5Z" fill="currentColor" /><path d="M4 10.5H9V14.5H15V10.5H20V19.5H4V10.5Z" fill="currentColor" /></svg>
   }
}