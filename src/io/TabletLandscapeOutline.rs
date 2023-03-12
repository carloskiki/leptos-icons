#[cfg(feature = "IoTabletLandscapeOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTabletLandscapeOutline")]
/// *This icon requires the feature* `IoTabletLandscapeOutline` *to be enabled*.
#[component]
pub fn TabletLandscapeOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="80" y="16" width="352" height="480" rx="48" ry="48" transform="translate(0 512) rotate(-90)" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}