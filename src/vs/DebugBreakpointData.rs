#[cfg(feature = "VsDebugBreakpointData")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugBreakpointData")]
/// *This icon requires the feature* `VsDebugBreakpointData` *to be enabled*.
#[component]
pub fn DebugBreakpointData(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M12.238 8l-2.31 4H5.31L3 8l2.31-4h4.618l2.31 4z" /></svg>
   }
}