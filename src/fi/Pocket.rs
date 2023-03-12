#[cfg(feature = "FiPocket")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPocket")]
/// *This icon requires the feature* `FiPocket` *to be enabled*.
#[component]
pub fn Pocket(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z" /><polyline points="8 10 12 14 16 10" /></svg>
   }
}