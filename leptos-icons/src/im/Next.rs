#[cfg(feature = "ImNext")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImNext")]
/// *This icon requires the feature* `ImNext` *to be enabled*.
#[component]
pub fn Next(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0c4.418 0 8 3.582 8 8s-3.582 8-8 8-8-3.582-8-8 3.582-8 8-8zM8 14.5c3.59 0 6.5-2.91 6.5-6.5s-2.91-6.5-6.5-6.5-6.5 2.91-6.5 6.5 2.91 6.5 6.5 6.5z" /><path fill="#000000" d="M9 8l-4-3v6z" /><path fill="#000000" d="M11 5h-2v6h2v-6z" /></svg>
   }
}