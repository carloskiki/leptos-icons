#[cfg(feature = "VsChevronUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsChevronUp")]
/// *This icon requires the feature* `VsChevronUp` *to be enabled*.
#[component]
pub fn ChevronUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8.024 5.928l-4.357 4.357-.62-.618L7.716 5h.618L13 9.667l-.619.618-4.357-4.357z" /></svg>
   }
}