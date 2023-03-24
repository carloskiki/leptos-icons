#[cfg(feature = "VsDebugStepInto")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugStepInto")]
/// *This icon requires the feature* `VsDebugStepInto` *to be enabled*.
#[component]
pub fn DebugStepInto(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 9.532h.542l3.905-3.905-1.061-1.06-2.637 2.61V1H7.251v6.177l-2.637-2.61-1.061 1.06 3.905 3.905H8zm1.956 3.481a2 2 0 1 1-4 0 2 2 0 0 1 4 0z" /></svg>
   }
}