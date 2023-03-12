#[cfg(feature = "ImArrowDownRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImArrowDownRight")]
/// *This icon requires the feature* `ImArrowDownRight` *to be enabled*.
#[component]
pub fn ArrowDownRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 4.5l-4 4-8.5-8.5-3.5 3.5 8.5 8.5-4 4h11.5v-11.5z" /></svg>
   }
}