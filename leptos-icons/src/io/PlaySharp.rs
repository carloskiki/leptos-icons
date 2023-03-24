#[cfg(feature = "IoPlaySharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlaySharp")]
/// *This icon requires the feature* `IoPlaySharp` *to be enabled*.
#[component]
pub fn PlaySharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="96 448 416 256 96 64 96 448" /></svg>
   }
}