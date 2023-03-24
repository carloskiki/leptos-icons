#[cfg(feature = "IoTriangleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTriangleOutline")]
/// *This icon requires the feature* `IoTriangleOutline` *to be enabled*.
#[component]
pub fn TriangleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="48 448 256 64 464 448 48 448" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}