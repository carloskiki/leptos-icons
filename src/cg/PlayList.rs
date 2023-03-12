#[cfg(feature = "CgPlayList")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayList")]
/// *This icon requires the feature* `CgPlayList` *to be enabled*.
#[component]
pub fn PlayList(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16 5H4V7H16V5Z" fill="currentColor" /><path d="M16 9H4V11H16V9Z" fill="currentColor" /><path d="M4 13H12V15H4V13Z" fill="currentColor" /><path d="M20 16L14 13V19L20 16Z" fill="currentColor" /></svg>
   }
}