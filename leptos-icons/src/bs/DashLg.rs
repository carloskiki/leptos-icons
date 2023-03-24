#[cfg(feature = "BsDashLg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsDashLg")]
/// *This icon requires the feature* `BsDashLg` *to be enabled*.
#[component]
pub fn DashLg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-dash-lg" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2 8a.5.5 0 0 1 .5-.5h11a.5.5 0 0 1 0 1h-11A.5.5 0 0 1 2 8Z" /></svg>
   }
}