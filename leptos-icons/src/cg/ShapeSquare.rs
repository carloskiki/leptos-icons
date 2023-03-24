#[cfg(feature = "CgShapeSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgShapeSquare")]
/// *This icon requires the feature* `CgShapeSquare` *to be enabled*.
#[component]
pub fn ShapeSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M17 7H7V17H17V7ZM4 4V20H20V4H4Z" fill="currentColor" /></svg>
   }
}