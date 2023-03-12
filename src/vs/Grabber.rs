#[cfg(feature = "VsGrabber")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsGrabber")]
/// *This icon requires the feature* `VsGrabber` *to be enabled*.
#[component]
pub fn Grabber(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M15 6H1v1h14V6zm0 3H1v1h14V9z" /></svg>
   }
}