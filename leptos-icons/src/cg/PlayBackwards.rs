#[cfg(feature = "CgPlayBackwards")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayBackwards")]
/// *This icon requires the feature* `CgPlayBackwards` *to be enabled*.
#[component]
pub fn PlayBackwards(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M2 7H5V17H2V7Z" fill="currentColor" /><path d="M6 12L13.0023 7.00003V17L6 12Z" fill="currentColor" /><path d="M21.0023 7.00003L14 12L21.0023 17V7.00003Z" fill="currentColor" /></svg>
   }
}