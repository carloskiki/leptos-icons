#[cfg(feature = "VsDebugBreakpointFunction")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugBreakpointFunction")]
/// *This icon requires the feature* `VsDebugBreakpointFunction` *to be enabled*.
#[component]
pub fn DebugBreakpointFunction(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 4l4 6.905H4L8 4z" /></svg>
   }
}