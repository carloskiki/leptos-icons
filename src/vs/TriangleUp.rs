#[cfg(feature = "VsTriangleUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsTriangleUp")]
/// *This icon requires the feature* `VsTriangleUp` *to be enabled*.
#[component]
pub fn TriangleUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14 10.44l-.413.56H2.393L2 10.46 7.627 5h.827L14 10.44z" /></svg>
   }
}