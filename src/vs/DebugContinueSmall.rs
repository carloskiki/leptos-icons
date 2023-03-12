#[cfg(feature = "VsDebugContinueSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugContinueSmall")]
/// *This icon requires the feature* `VsDebugContinueSmall` *to be enabled*.
#[component]
pub fn DebugContinueSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4 2H3V14H4V2ZM7.29062 2.59314L6.5 3.00001V13L7.29062 13.4069L14.2906 8.40687V7.59314L7.29062 2.59314ZM13.1398 8.00001L7.5 12.0284V3.9716L13.1398 8.00001Z" /></svg>
   }
}