#[cfg(feature = "FiChevronsLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiChevronsLeft")]
/// *This icon requires the feature* `FiChevronsLeft` *to be enabled*.
#[component]
pub fn ChevronsLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="11 17 6 12 11 7" /><polyline points="18 17 13 12 18 7" /></svg>
   }
}