#[cfg(feature = "VsCircleSmallFilled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsCircleSmallFilled")]
/// *This icon requires the feature* `VsCircleSmallFilled` *to be enabled*.
#[component]
pub fn CircleSmallFilled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"><path d="M10 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0z" /></svg>
   }
}