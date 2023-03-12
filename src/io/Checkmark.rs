#[cfg(feature = "IoCheckmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCheckmark")]
/// *This icon requires the feature* `IoCheckmark` *to be enabled*.
#[component]
pub fn Checkmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="416 128 192 384 96 288" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}