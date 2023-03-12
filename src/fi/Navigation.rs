#[cfg(feature = "FiNavigation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiNavigation")]
/// *This icon requires the feature* `FiNavigation` *to be enabled*.
#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="3 11 22 2 13 21 11 13 3 11" /></svg>
   }
}