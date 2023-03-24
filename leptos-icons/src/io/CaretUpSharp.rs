#[cfg(feature = "IoCaretUpSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretUpSharp")]
/// *This icon requires the feature* `IoCaretUpSharp` *to be enabled*.
#[component]
pub fn CaretUpSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="448 368 256 144 64 368 448 368" /></svg>
   }
}