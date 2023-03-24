#[cfg(feature = "FiMinimize2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMinimize2")]
/// *This icon requires the feature* `FiMinimize2` *to be enabled*.
#[component]
pub fn Minimize2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 14 10 14 10 20" /><polyline points="20 10 14 10 14 4" /><line x1="14" y1="10" x2="21" y2="3" /><line x1="3" y1="21" x2="10" y2="14" /></svg>
   }
}