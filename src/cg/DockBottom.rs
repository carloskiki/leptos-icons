#[cfg(feature = "CgDockBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDockBottom")]
/// *This icon requires the feature* `CgDockBottom` *to be enabled*.
#[component]
pub fn DockBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 20V4H22V20H2ZM4 6H20V14H4V6Z" fill="currentColor" /></svg>
   }
}