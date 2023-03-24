#[cfg(feature = "FiBarChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiBarChart")]
/// *This icon requires the feature* `FiBarChart` *to be enabled*.
#[component]
pub fn BarChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="20" x2="12" y2="10" /><line x1="18" y1="20" x2="18" y2="4" /><line x1="6" y1="20" x2="6" y2="16" /></svg>
   }
}