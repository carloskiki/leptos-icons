#[cfg(feature = "ImArrowUpRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowUpRight")]
/// *This icon requires the feature* `ImArrowUpRight` *to be enabled*.
#[component]
pub fn ArrowUpRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4.5 0l4 4-8.5 8.5 3.5 3.5 8.5-8.5 4 4v-11.5h-11.5z" /></svg>
   }
}