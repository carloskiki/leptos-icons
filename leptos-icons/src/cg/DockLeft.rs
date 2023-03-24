#[cfg(feature = "CgDockLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDockLeft")]
/// *This icon requires the feature* `CgDockLeft` *to be enabled*.
#[component]
pub fn DockLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 4H22V20H2V4ZM8 6H20V18H8V6Z" fill="currentColor" /></svg>
   }
}