#[cfg(feature = "ImShrink")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImShrink")]
/// *This icon requires the feature* `ImShrink` *to be enabled*.
#[component]
pub fn Shrink(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 7h6.5l-2.5-2.5 3-3-1.5-1.5-3 3-2.5-2.5z" /><path fill="#000000" d="M9 9v6.5l2.5-2.5 3 3 1.5-1.5-3-3 2.5-2.5z" /><path fill="#000000" d="M7 9h-6.5l2.5 2.5-3 3 1.5 1.5 3-3 2.5 2.5z" /><path fill="#000000" d="M7 7v-6.5l-2.5 2.5-3-3-1.5 1.5 3 3-2.5 2.5z" /></svg>
   }
}