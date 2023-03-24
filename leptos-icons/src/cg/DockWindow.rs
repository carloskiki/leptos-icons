#[cfg(feature = "CgDockWindow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDockWindow")]
/// *This icon requires the feature* `CgDockWindow` *to be enabled*.
#[component]
pub fn DockWindow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 20V4H22V20H2ZM20 6H6V16L20 16V6Z" fill="currentColor" /></svg>
   }
}