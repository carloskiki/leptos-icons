#[cfg(feature = "ImSuperscript")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImSuperscript")]
/// *This icon requires the feature* `ImSuperscript` *to be enabled*.
#[component]
pub fn Superscript(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M12 3.219v0.781h2v1h-3v-2.281l2-0.938v-0.781h-2v-1h3v2.281zM10.563 4h-2.125l-2.938 2.938-2.938-2.938h-2.125l4 4-4 4h2.125l2.938-2.938 2.938 2.938h2.125l-4-4z" /></svg>
   }
}