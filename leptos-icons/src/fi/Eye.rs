#[cfg(feature = "FiEye")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiEye")]
/// *This icon requires the feature* `FiEye` *to be enabled*.
#[component]
pub fn Eye(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" /><circle cx="12" cy="12" r="3" /></svg>
   }
}