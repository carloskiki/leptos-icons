#[cfg(feature = "IoArrowUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowUp")]
/// *This icon requires the feature* `IoArrowUp` *to be enabled*.
#[component]
pub fn ArrowUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="112 244 256 100 400 244" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /><line x1="256" y1="120" x2="256" y2="412" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}