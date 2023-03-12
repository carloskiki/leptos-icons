#[cfg(feature = "VsDebugPause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugPause")]
/// *This icon requires the feature* `VsDebugPause` *to be enabled*.
#[component]
pub fn DebugPause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4.5 3H6v10H4.5V3zm7 0v10H10V3h1.5z" /></svg>
   }
}