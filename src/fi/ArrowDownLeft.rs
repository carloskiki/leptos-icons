#[cfg(feature = "FiArrowDownLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiArrowDownLeft")]
/// *This icon requires the feature* `FiArrowDownLeft` *to be enabled*.
#[component]
pub fn ArrowDownLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="17" y1="7" x2="7" y2="17" /><polyline points="17 17 7 17 7 7" /></svg>
   }
}