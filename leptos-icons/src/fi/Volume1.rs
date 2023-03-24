#[cfg(feature = "FiVolume1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiVolume1")]
/// *This icon requires the feature* `FiVolume1` *to be enabled*.
#[component]
pub fn Volume1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" /><path d="M15.54 8.46a5 5 0 0 1 0 7.07" /></svg>
   }
}