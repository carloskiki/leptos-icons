#[cfg(feature = "CgLoadbarSound")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgLoadbarSound")]
/// *This icon requires the feature* `CgLoadbarSound` *to be enabled*.
#[component]
pub fn LoadbarSound(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11 6H13V18H11V6Z" fill="currentColor" /><path d="M7 13H9V18H7V13Z" fill="currentColor" /><path d="M15 9H17V18H15V9Z" fill="currentColor" /></svg>
   }
}