#[cfg(feature = "FiActivity")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiActivity")]
/// *This icon requires the feature* `FiActivity` *to be enabled*.
#[component]
pub fn Activity(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12" /></svg>
   }
}