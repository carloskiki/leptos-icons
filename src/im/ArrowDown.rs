#[cfg(feature = "ImArrowDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowDown")]
/// *This icon requires the feature* `ImArrowDown` *to be enabled*.
#[component]
pub fn ArrowDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 15.5l7.5-7.5h-4.5v-8h-6v8h-4.5z" /></svg>
   }
}