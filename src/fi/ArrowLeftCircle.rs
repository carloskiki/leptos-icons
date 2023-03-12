#[cfg(feature = "FiArrowLeftCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiArrowLeftCircle")]
/// *This icon requires the feature* `FiArrowLeftCircle` *to be enabled*.
#[component]
pub fn ArrowLeftCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><polyline points="12 8 8 12 12 16" /><line x1="16" y1="12" x2="8" y2="12" /></svg>
   }
}