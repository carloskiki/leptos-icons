#[cfg(feature = "FiClock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiClock")]
/// *This icon requires the feature* `FiClock` *to be enabled*.
#[component]
pub fn Clock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><polyline points="12 6 12 12 16 14" /></svg>
   }
}