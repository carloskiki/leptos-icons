#[cfg(feature = "CgSwiss")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSwiss")]
/// *This icon requires the feature* `CgSwiss` *to be enabled*.
#[component]
pub fn Swiss(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 3V21H21V3H3ZM14 7H10V10H7V14H10V17H14V14H17V10H14V7Z" fill="currentColor" /></svg>
   }
}