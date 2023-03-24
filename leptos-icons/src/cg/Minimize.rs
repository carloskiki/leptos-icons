#[cfg(feature = "CgMinimize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMinimize")]
/// *This icon requires the feature* `CgMinimize` *to be enabled*.
#[component]
pub fn Minimize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9 9H3V7H7V3H9V9Z" fill="currentColor" /><path d="M9 15H3V17H7V21H9V15Z" fill="currentColor" /><path d="M21 15H15V21H17V17H21V15Z" fill="currentColor" /><path d="M15 9.00012H21V7.00012H17V3.00012H15V9.00012Z" fill="currentColor" /></svg>
   }
}