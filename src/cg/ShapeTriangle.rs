#[cfg(feature = "CgShapeTriangle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgShapeTriangle")]
/// *This icon requires the feature* `CgShapeTriangle` *to be enabled*.
#[component]
pub fn ShapeTriangle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M11.6603 5L3 20H20.3205L11.6603 5ZM11.6603 11L8.19615 17H15.1244L11.6603 11Z" fill="currentColor" /></svg>
   }
}