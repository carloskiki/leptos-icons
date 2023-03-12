#[cfg(feature = "FiSmartphone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiSmartphone")]
/// *This icon requires the feature* `FiSmartphone` *to be enabled*.
#[component]
pub fn Smartphone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="2" width="14" height="20" rx="2" ry="2" /><line x1="12" y1="18" x2="12.01" y2="18" /></svg>
   }
}