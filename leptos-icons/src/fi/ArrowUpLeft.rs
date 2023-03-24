#[cfg(feature = "FiArrowUpLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiArrowUpLeft")]
/// *This icon requires the feature* `FiArrowUpLeft` *to be enabled*.
#[component]
pub fn ArrowUpLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="17" y1="17" x2="7" y2="7" /><polyline points="7 17 7 7 17 7" /></svg>
   }
}