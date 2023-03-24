#[cfg(feature = "CgBorderLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBorderLeft")]
/// *This icon requires the feature* `CgBorderLeft` *to be enabled*.
#[component]
pub fn BorderLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16 8V16H9L9 19H19L19 5L9 5V8H16Z" fill="currentColor" fill-opacity="0.3" /><path d="M7 5L7 19H4L4 5L7 5Z" fill="currentColor" /></svg>
   }
}