#[cfg(feature = "FiCompass")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCompass")]
/// *This icon requires the feature* `FiCompass` *to be enabled*.
#[component]
pub fn Compass(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" /></svg>
   }
}