#[cfg(feature = "FiStopCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiStopCircle")]
/// *This icon requires the feature* `FiStopCircle` *to be enabled*.
#[component]
pub fn StopCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><rect x="9" y="9" width="6" height="6" /></svg>
   }
}