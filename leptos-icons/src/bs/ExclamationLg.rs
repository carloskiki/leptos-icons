#[cfg(feature = "BsExclamationLg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsExclamationLg")]
/// *This icon requires the feature* `BsExclamationLg` *to be enabled*.
#[component]
pub fn ExclamationLg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-exclamation-lg" viewBox="0 0 16 16"><path d="M7.005 3.1a1 1 0 1 1 1.99 0l-.388 6.35a.61.61 0 0 1-1.214 0L7.005 3.1ZM7 12a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z" /></svg>
   }
}