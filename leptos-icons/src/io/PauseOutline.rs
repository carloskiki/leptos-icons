#[cfg(feature = "IoPauseOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPauseOutline")]
/// *This icon requires the feature* `IoPauseOutline` *to be enabled*.
#[component]
pub fn PauseOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="176" y="96" width="16" height="320" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><rect x="320" y="96" width="16" height="320" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}