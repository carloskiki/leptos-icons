#[cfg(feature = "IoCodeOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCodeOutline")]
/// *This icon requires the feature* `IoCodeOutline` *to be enabled*.
#[component]
pub fn CodeOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="160 368 32 256 160 144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="352 368 480 256 352 144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}