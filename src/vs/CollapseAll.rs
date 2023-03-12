#[cfg(feature = "VsCollapseAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsCollapseAll")]
/// *This icon requires the feature* `VsCollapseAll` *to be enabled*.
#[component]
pub fn CollapseAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M9 9H4v1h5V9z" /><path fill-rule="evenodd" clip-rule="evenodd" d="M5 3l1-1h7l1 1v7l-1 1h-2v2l-1 1H3l-1-1V6l1-1h2V3zm1 2h4l1 1v4h2V3H6v2zm4 1H3v7h7V6z" /></svg>
   }
}