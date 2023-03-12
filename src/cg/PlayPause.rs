#[cfg(feature = "CgPlayPause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayPause")]
/// *This icon requires the feature* `CgPlayPause` *to be enabled*.
#[component]
pub fn PlayPause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11 7H8V17H11V7Z" fill="currentColor" /><path d="M13 17H16V7H13V17Z" fill="currentColor" /></svg>
   }
}