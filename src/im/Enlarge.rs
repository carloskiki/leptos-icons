#[cfg(feature = "ImEnlarge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImEnlarge")]
/// *This icon requires the feature* `ImEnlarge` *to be enabled*.
#[component]
pub fn Enlarge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 0h-6.5l2.5 2.5-3 3 1.5 1.5 3-3 2.5 2.5z" /><path fill="#000000" d="M16 16v-6.5l-2.5 2.5-3-3-1.5 1.5 3 3-2.5 2.5z" /><path fill="#000000" d="M0 16h6.5l-2.5-2.5 3-3-1.5-1.5-3 3-2.5-2.5z" /><path fill="#000000" d="M0 0v6.5l2.5-2.5 3 3 1.5-1.5-3-3 2.5-2.5z" /></svg>
   }
}