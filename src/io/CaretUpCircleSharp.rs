#[cfg(feature = "IoCaretUpCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretUpCircleSharp")]
/// *This icon requires the feature* `IoCaretUpCircleSharp` *to be enabled*.
#[component]
pub fn CaretUpCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48ZM147.73,300,256,169.91,364.27,300Z" /></svg>
   }
}