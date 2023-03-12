#[cfg(feature = "IoStopOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoStopOutline")]
/// *This icon requires the feature* `IoStopOutline` *to be enabled*.
#[component]
pub fn StopOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="96" y="96" width="320" height="320" rx="24" ry="24" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}