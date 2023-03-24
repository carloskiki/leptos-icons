#[cfg(feature = "VsDebugStart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugStart")]
/// *This icon requires the feature* `VsDebugStart` *to be enabled*.
#[component]
pub fn DebugStart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4.25 3l1.166-.624 8 5.333v1.248l-8 5.334-1.166-.624V3zm1.5 1.401v7.864l5.898-3.932L5.75 4.401z" /></svg>
   }
}