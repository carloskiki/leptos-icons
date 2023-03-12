#[cfg(feature = "VsEllipsis")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsEllipsis")]
/// *This icon requires the feature* `VsEllipsis` *to be enabled*.
#[component]
pub fn Ellipsis(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4 8a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm5 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0zm5 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0z" /></svg>
   }
}