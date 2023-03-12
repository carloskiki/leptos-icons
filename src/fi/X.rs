#[cfg(feature = "FiX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiX")]
/// *This icon requires the feature* `FiX` *to be enabled*.
#[component]
pub fn X(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg>
   }
}