#[cfg(feature = "VsTriangleLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTriangleLeft")]
/// *This icon requires the feature* `VsTriangleLeft` *to be enabled*.
#[component]
pub fn TriangleLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M10.44 2l.56.413v11.194l-.54.393L5 8.373v-.827L10.44 2z" /></svg>
   }
}