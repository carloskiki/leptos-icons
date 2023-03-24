#[cfg(feature = "CgShapeHexagon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgShapeHexagon")]
/// *This icon requires the feature* `CgShapeHexagon` *to be enabled*.
#[component]
pub fn ShapeHexagon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M6 15.2348L12 18.5681L18 15.2348V8.76521L12 5.43188L6 8.76521V15.2348ZM12 2L3 7V17L12 22L21 17V7L12 2Z" fill="currentColor" /></svg>
   }
}