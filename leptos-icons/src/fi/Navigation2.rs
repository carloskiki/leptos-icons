#[cfg(feature = "FiNavigation2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiNavigation2")]
/// *This icon requires the feature* `FiNavigation2` *to be enabled*.
#[component]
pub fn Navigation2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 19 21 12 17 5 21 12 2" /></svg>
   }
}