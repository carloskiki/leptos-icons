#[cfg(feature = "IoEllipseOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEllipseOutline")]
/// *This icon requires the feature* `IoEllipseOutline` *to be enabled*.
#[component]
pub fn EllipseOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="192" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}