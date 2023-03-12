#[cfg(feature = "CgShapeRhombus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgShapeRhombus")]
/// *This icon requires the feature* `CgShapeRhombus` *to be enabled*.
#[component]
pub fn ShapeRhombus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 6.34315L6.34317 12L12 17.6569L17.6569 12L12 6.34315ZM2.10052 12L12 21.8995L21.8995 12L12 2.10051L2.10052 12Z" fill="currentColor" /></svg>
   }
}