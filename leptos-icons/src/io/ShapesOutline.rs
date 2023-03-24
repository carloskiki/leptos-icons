#[cfg(feature = "IoShapesOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoShapesOutline")]
/// *This icon requires the feature* `IoShapesOutline` *to be enabled*.
#[component]
pub fn ShapesOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="336 320 32 320 184 48 336 320" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M265.32,194.51A144,144,0,1,1,192,320" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}