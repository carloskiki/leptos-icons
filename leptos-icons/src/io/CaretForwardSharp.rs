#[cfg(feature = "IoCaretForwardSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretForwardSharp")]
/// *This icon requires the feature* `IoCaretForwardSharp` *to be enabled*.
#[component]
pub fn CaretForwardSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="144 448 368 256 144 64 144 448" /></svg>
   }
}