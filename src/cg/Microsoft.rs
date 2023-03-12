#[cfg(feature = "CgMicrosoft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMicrosoft")]
/// *This icon requires the feature* `CgMicrosoft` *to be enabled*.
#[component]
pub fn Microsoft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 3H11V11H3V3Z" fill="currentColor" /><path d="M3 13H11V21H3V13Z" fill="currentColor" /><path d="M13 3H21V11H13V3Z" fill="currentColor" /><path d="M13 13H21V21H13V13Z" fill="currentColor" /></svg>
   }
}