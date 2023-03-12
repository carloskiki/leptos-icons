#[cfg(feature = "FiSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiSquare")]
/// *This icon requires the feature* `FiSquare` *to be enabled*.
#[component]
pub fn Square(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2" /></svg>
   }
}