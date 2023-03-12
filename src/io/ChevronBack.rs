#[cfg(feature = "IoChevronBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronBack")]
/// *This icon requires the feature* `IoChevronBack` *to be enabled*.
#[component]
pub fn ChevronBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="328 112 184 256 328 400" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}