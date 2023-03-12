#[cfg(feature = "IoTriangleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTriangleSharp")]
/// *This icon requires the feature* `IoTriangleSharp` *to be enabled*.
#[component]
pub fn TriangleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="256 32 20 464 492 464 256 32" /></svg>
   }
}