#[cfg(feature = "ImArrowLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowLeft")]
/// *This icon requires the feature* `ImArrowLeft` *to be enabled*.
#[component]
pub fn ArrowLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0.5 8l7.5 7.5v-4.5h8v-6h-8v-4.5z" /></svg>
   }
}