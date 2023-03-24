#[cfg(feature = "VsKebabVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsKebabVertical")]
/// *This icon requires the feature* `VsKebabVertical` *to be enabled*.
#[component]
pub fn KebabVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M7.444 13.832a1 1 0 1 0 1.111-1.663 1 1 0 0 0-1.11 1.662zM8 9a1 1 0 1 1 0-2 1 1 0 0 1 0 2zm0-5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z" /></svg>
   }
}