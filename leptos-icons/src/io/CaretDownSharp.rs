#[cfg(feature = "IoCaretDownSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretDownSharp")]
/// *This icon requires the feature* `IoCaretDownSharp` *to be enabled*.
#[component]
pub fn CaretDownSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="64 144 256 368 448 144 64 144" /></svg>
   }
}