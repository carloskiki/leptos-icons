#[cfg(feature = "VsTriangleRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTriangleRight")]
/// *This icon requires the feature* `VsTriangleRight` *to be enabled*.
#[component]
pub fn TriangleRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M5.56 14L5 13.587V2.393L5.54 2 11 7.627v.827L5.56 14z" /></svg>
   }
}