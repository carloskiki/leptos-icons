#[cfg(feature = "FiArrowUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiArrowUp")]
/// *This icon requires the feature* `FiArrowUp` *to be enabled*.
#[component]
pub fn ArrowUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="19" x2="12" y2="5" /><polyline points="5 12 12 5 19 12" /></svg>
   }
}