#[cfg(feature = "VsDebugBreakpointUnsupported")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugBreakpointUnsupported")]
/// *This icon requires the feature* `VsDebugBreakpointUnsupported` *to be enabled*.
#[component]
pub fn DebugBreakpointUnsupported(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M11.326 10.222a4 4 0 1 0-6.653-4.444 4 4 0 0 0 6.653 4.444zM8.65 10H7.4v1h1.25v-1zM7.4 9V5h1.25v4H7.4z" /></svg>
   }
}