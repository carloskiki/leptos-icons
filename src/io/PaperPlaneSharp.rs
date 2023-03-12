#[cfg(feature = "IoPaperPlaneSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPaperPlaneSharp")]
/// *This icon requires the feature* `IoPaperPlaneSharp` *to be enabled*.
#[component]
pub fn PaperPlaneSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="496 16 15.88 208 195 289 448 64 223 317 304 496 496 16" /></svg>
   }
}