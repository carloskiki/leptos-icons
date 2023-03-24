#[cfg(feature = "IoFlashSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFlashSharp")]
/// *This icon requires the feature* `IoFlashSharp` *to be enabled*.
#[component]
pub fn FlashSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M432,208H288L320,16,80,304H224L192,496Z" /></svg>
   }
}