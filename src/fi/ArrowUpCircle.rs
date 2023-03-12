#[cfg(feature = "FiArrowUpCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiArrowUpCircle")]
/// *This icon requires the feature* `FiArrowUpCircle` *to be enabled*.
#[component]
pub fn ArrowUpCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><polyline points="16 12 12 8 8 12" /><line x1="12" y1="16" x2="12" y2="8" /></svg>
   }
}