#[cfg(feature = "ImArrowRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowRight")]
/// *This icon requires the feature* `ImArrowRight` *to be enabled*.
#[component]
pub fn ArrowRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M15.5 8l-7.5-7.5v4.5h-8v6h8v4.5z" /></svg>
   }
}