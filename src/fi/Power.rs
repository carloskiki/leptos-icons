#[cfg(feature = "FiPower")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPower")]
/// *This icon requires the feature* `FiPower` *to be enabled*.
#[component]
pub fn Power(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18.36 6.64a9 9 0 1 1-12.73 0" /><line x1="12" y1="2" x2="12" y2="12" /></svg>
   }
}