#[cfg(feature = "OcSmTriangleUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmTriangleUp")]
/// *This icon requires the feature* `OcSmTriangleUp` *to be enabled*.
#[component]
pub fn TriangleUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="m4.427 9.573 3.396-3.396a.25.25 0 0 1 .354 0l3.396 3.396a.25.25 0 0 1-.177.427H4.604a.25.25 0 0 1-.177-.427Z" /></svg>
   }
}