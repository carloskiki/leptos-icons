#[cfg(feature = "IoChevronDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronDown")]
/// *This icon requires the feature* `IoChevronDown` *to be enabled*.
#[component]
pub fn ChevronDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="112 184 256 328 400 184" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}