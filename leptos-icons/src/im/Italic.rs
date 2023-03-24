#[cfg(feature = "ImItalic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImItalic")]
/// *This icon requires the feature* `ImItalic` *to be enabled*.
#[component]
pub fn Italic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 1v1h-2l-5 12h2v1h-7v-1h2l5-12h-2v-1z" /></svg>
   }
}