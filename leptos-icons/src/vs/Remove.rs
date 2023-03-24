#[cfg(feature = "VsRemove")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRemove")]
/// *This icon requires the feature* `VsRemove` *to be enabled*.
#[component]
pub fn Remove(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M15 8H1V7h14v1z" /></svg>
   }
}