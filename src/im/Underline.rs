#[cfg(feature = "ImUnderline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImUnderline")]
/// *This icon requires the feature* `ImUnderline` *to be enabled*.
#[component]
pub fn Underline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11 1h2v6.5c0 2.485-2.239 4.5-5 4.5s-5-2.015-5-4.5v-6.5h2v6.5c0 0.628 0.285 1.23 0.802 1.695 0.577 0.519 1.357 0.805 2.198 0.805s1.621-0.286 2.198-0.805c0.517-0.466 0.802-1.068 0.802-1.695v-6.5zM3 13h10v2h-10z" /></svg>
   }
}