#[cfg(feature = "IoReturnUpBackSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReturnUpBackSharp")]
/// *This icon requires the feature* `IoReturnUpBackSharp` *to be enabled*.
#[component]
pub fn ReturnUpBackSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="112 160 48 224 112 288" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><polyline points="64 224 464 224 464 352" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}