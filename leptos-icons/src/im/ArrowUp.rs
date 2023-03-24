#[cfg(feature = "ImArrowUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowUp")]
/// *This icon requires the feature* `ImArrowUp` *to be enabled*.
#[component]
pub fn ArrowUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0.5l-7.5 7.5h4.5v8h6v-8h4.5z" /></svg>
   }
}