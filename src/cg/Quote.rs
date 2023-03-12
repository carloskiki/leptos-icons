#[cfg(feature = "CgQuote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgQuote")]
/// *This icon requires the feature* `CgQuote` *to be enabled*.
#[component]
pub fn Quote(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9.13456 9H12.1346L10 14.6075H7L9.13456 9Z" fill="currentColor" /><path d="M14.1346 9H17.1346L15 14.6075H12L14.1346 9Z" fill="currentColor" /></svg>
   }
}