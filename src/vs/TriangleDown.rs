#[cfg(feature = "VsTriangleDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTriangleDown")]
/// *This icon requires the feature* `VsTriangleDown` *to be enabled*.
#[component]
pub fn TriangleDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 5.56L2.413 5h11.194l.393.54L8.373 11h-.827L2 5.56z" /></svg>
   }
}