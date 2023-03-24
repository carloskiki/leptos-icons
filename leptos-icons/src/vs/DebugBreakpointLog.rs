#[cfg(feature = "VsDebugBreakpointLog")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugBreakpointLog")]
/// *This icon requires the feature* `VsDebugBreakpointLog` *to be enabled*.
#[component]
pub fn DebugBreakpointLog(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 3l5 5-5 5-5-5 5-5z" /></svg>
   }
}