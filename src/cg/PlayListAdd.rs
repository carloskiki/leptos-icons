#[cfg(feature = "CgPlayListAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayListAdd")]
/// *This icon requires the feature* `CgPlayListAdd` *to be enabled*.
#[component]
pub fn PlayListAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M2 5H14V7H2V5Z" fill="currentColor" /><path d="M2 9H14V11H2V9Z" fill="currentColor" /><path d="M10 13H2V15H10V13Z" fill="currentColor" /><path d="M16 9H18V13H22V15H18V19H16V15H12V13H16V9Z" fill="currentColor" /></svg>
   }
}