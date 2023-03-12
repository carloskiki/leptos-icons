#[cfg(feature = "CgLoadbarAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgLoadbarAlt")]
/// *This icon requires the feature* `CgLoadbarAlt` *to be enabled*.
#[component]
pub fn LoadbarAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><rect opacity="0.3" x="3" y="10" width="18" height="4" rx="2" fill="currentColor" /><rect x="7" y="10" width="10" height="4" rx="2" fill="currentColor" /></svg>
   }
}