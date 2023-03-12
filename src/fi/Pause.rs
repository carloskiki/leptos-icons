#[cfg(feature = "FiPause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPause")]
/// *This icon requires the feature* `FiPause` *to be enabled*.
#[component]
pub fn Pause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="6" y="4" width="4" height="16" /><rect x="14" y="4" width="4" height="16" /></svg>
   }
}