#[cfg(feature = "CgPlayTrackPrev")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayTrackPrev")]
/// *This icon requires the feature* `CgPlayTrackPrev` *to be enabled*.
#[component]
pub fn PlayTrackPrev(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M18 17L10 12L18 7V17Z" fill="currentColor" /><path d="M6 7H9V17H6V7Z" fill="currentColor" /></svg>
   }
}