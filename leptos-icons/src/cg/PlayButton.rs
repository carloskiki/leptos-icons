#[cfg(feature = "CgPlayButton")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayButton")]
/// *This icon requires the feature* `CgPlayButton` *to be enabled*.
#[component]
pub fn PlayButton(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15 12.3301L9 16.6603L9 8L15 12.3301Z" fill="currentColor" /></svg>
   }
}