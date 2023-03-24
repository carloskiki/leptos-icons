#[cfg(feature = "CgDockRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDockRight")]
/// *This icon requires the feature* `CgDockRight` *to be enabled*.
#[component]
pub fn DockRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M2 4H22V20H2V4ZM16 18V6H4V18H16Z" fill="currentColor" /></svg>
   }
}