#[cfg(feature = "IoSquareSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSquareSharp")]
/// *This icon requires the feature* `IoSquareSharp` *to be enabled*.
#[component]
pub fn SquareSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="48" y="48" width="416" height="416" /></svg>
   }
}