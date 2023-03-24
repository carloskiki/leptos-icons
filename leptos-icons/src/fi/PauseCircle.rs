#[cfg(feature = "FiPauseCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPauseCircle")]
/// *This icon requires the feature* `FiPauseCircle` *to be enabled*.
#[component]
pub fn PauseCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><line x1="10" y1="15" x2="10" y2="9" /><line x1="14" y1="15" x2="14" y2="9" /></svg>
   }
}