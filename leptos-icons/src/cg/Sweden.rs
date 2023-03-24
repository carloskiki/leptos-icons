#[cfg(feature = "CgSweden")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSweden")]
/// *This icon requires the feature* `CgSweden` *to be enabled*.
#[component]
pub fn Sweden(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M23 4H10V11H23V4Z" fill="currentColor" /><path d="M23 13V20H10V13H23Z" fill="currentColor" /><path d="M8 13V20H1V13H8Z" fill="currentColor" /><path d="M1 11V4H8V11H1Z" fill="currentColor" /></svg>
   }
}