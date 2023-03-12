#[cfg(feature = "ImNeutral")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImNeutral")]
/// *This icon requires the feature* `ImNeutral` *to be enabled*.
#[component]
pub fn Neutral(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 16c4.418 0 8-3.582 8-8s-3.582-8-8-8-8 3.582-8 8 3.582 8 8 8zM8 1.5c3.59 0 6.5 2.91 6.5 6.5s-2.91 6.5-6.5 6.5-6.5-2.91-6.5-6.5 2.91-6.5 6.5-6.5zM4 5c0 0.552 0.448 1 1 1s1-0.448 1-1-0.448-1-1-1-1 0.448-1 1zM10 5c0 0.552 0.448 1 1 1s1-0.448 1-1-0.448-1-1-1-1 0.448-1 1zM6 11h4v1h-4v-1z" /></svg>
   }
}