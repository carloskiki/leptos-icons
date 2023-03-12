#[cfg(feature = "FiChevronsUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiChevronsUp")]
/// *This icon requires the feature* `FiChevronsUp` *to be enabled*.
#[component]
pub fn ChevronsUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="17 11 12 6 7 11" /><polyline points="17 18 12 13 7 18" /></svg>
   }
}