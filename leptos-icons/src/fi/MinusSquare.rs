#[cfg(feature = "FiMinusSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMinusSquare")]
/// *This icon requires the feature* `FiMinusSquare` *to be enabled*.
#[component]
pub fn MinusSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2" /><line x1="8" y1="12" x2="16" y2="12" /></svg>
   }
}