#[cfg(feature = "CgLoadbar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgLoadbar")]
/// *This icon requires the feature* `CgLoadbar` *to be enabled*.
#[component]
pub fn Loadbar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><rect x="3" y="10" width="18" height="4" rx="2" fill="currentColor" /></svg>
   }
}