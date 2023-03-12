#[cfg(feature = "IoCaretBackSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretBackSharp")]
/// *This icon requires the feature* `IoCaretBackSharp` *to be enabled*.
#[component]
pub fn CaretBackSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="368 64 144 256 368 448 368 64" /></svg>
   }
}