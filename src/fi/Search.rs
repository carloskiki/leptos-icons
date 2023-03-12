#[cfg(feature = "FiSearch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiSearch")]
/// *This icon requires the feature* `FiSearch` *to be enabled*.
#[component]
pub fn Search(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" /></svg>
   }
}