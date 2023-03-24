#[cfg(feature = "CgPlayStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayStop")]
/// *This icon requires the feature* `CgPlayStop` *to be enabled*.
#[component]
pub fn PlayStop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M7 7H17V17H7V7Z" fill="currentColor" /></svg>
   }
}