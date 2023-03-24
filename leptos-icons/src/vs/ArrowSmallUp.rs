#[cfg(feature = "VsArrowSmallUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowSmallUp")]
/// *This icon requires the feature* `VsArrowSmallUp` *to be enabled*.
#[component]
pub fn ArrowSmallUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M5 6.5L7.5 4h.7l2.5 2.5-.7.71-1.65-1.64v5.57h-1V5.57L5.7 7.22 5 6.5z" /></svg>
   }
}