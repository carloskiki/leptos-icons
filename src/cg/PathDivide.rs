#[cfg(feature = "CgPathDivide")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathDivide")]
/// *This icon requires the feature* `CgPathDivide` *to be enabled*.
#[component]
pub fn PathDivide(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 5H15V9H9V15H5V5Z" fill="currentColor" /><path d="M9 15V19H19V9H15V15H9Z" fill="currentColor" /><path d="M10 10H14V14H10V10Z" fill="currentColor" /></svg>
   }
}